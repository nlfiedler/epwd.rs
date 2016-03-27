/*
 * Copyright 2016 Nathan Fiedler
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[macro_use]
extern crate ruster_unsafe;
extern crate users;
extern crate libc;

use ruster_unsafe::*;
use std::ffi::CString;
use std::mem::uninitialized;
use libc::{c_uchar, uid_t};
use users::os::unix::UserExt;

/// Create NIF module data and init function.
nif_init!(b"epwd_rs\0", None, None, None, None,
     nif!(b"getpwnam\0", 1, getpwnam, 0),
     nif!(b"getpwuid\0", 1, getpwuid, 0)
    );

/// Retrieve the details for the named user as a property list.
extern "C" fn getpwnam(env: *mut ErlNifEnv,
                       argc: c_int,
                       args: *const ERL_NIF_TERM) -> ERL_NIF_TERM {
    if argc == 1 {
        // need to allocate the space for the incoming string
        // (256 should be enough for anybody)
        let mut name:Vec<c_uchar> = Vec::with_capacity(256);
        let name_len = unsafe { enif_get_string(env, *args.offset(0), name.as_mut_ptr(), 256,
            ErlNifCharEncoding::ERL_NIF_LATIN1) };
        if name_len == 0 {
            return make_err_result(env, "invalid name argument");
        }
        unsafe { name.set_len((name_len - 1) as usize) };
        let rname = std::str::from_utf8(&name);
        if rname.is_err() {
            return make_err_result(env, "invalid name");
        }
        match users::get_user_by_name(rname.unwrap()) {
            Some(user) => {
                let result = make_user_proplist(env, user);
                make_ok_result(env, &result)
            },
            None => make_err_result(env, "no such user")
        }
    } else {
        unsafe { enif_make_badarg(env) }
    }
}

/// Retrieve the details for the identified user as a property list.
extern "C" fn getpwuid(env: *mut ErlNifEnv,
                       argc: c_int,
                       args: *const ERL_NIF_TERM) -> ERL_NIF_TERM {
    let mut uid:c_int = unsafe { uninitialized() };
    if argc == 1 &&
       0 != unsafe { enif_get_int(env, *args.offset(0), &mut uid) } {
        match users::get_user_by_uid(uid as uid_t) {
            Some(user) => {
                let result = make_user_proplist(env, user);
                make_ok_result(env, &result)
            },
            None => make_err_result(env, "no such user")
        }
    } else {
        unsafe { enif_make_badarg(env) }
    }
}

/// Produce a property list consisting of the details of the given user.
/// Keys include pw_uid, pw_gid, pw_name, pw_dir, and pw_shell.
fn make_user_proplist(env: *mut ErlNifEnv, user: users::User) -> ERL_NIF_TERM {
    let uid = unsafe { enif_make_uint(env, user.uid()) };
    let uid_tuple = make_tuple(env, "pw_uid", &uid);
    let gid = unsafe { enif_make_uint(env, user.primary_group_id()) };
    let gid_tuple = make_tuple(env, "pw_gid", &gid);
    let user_name = user.name();
    let name_str = unsafe { enif_make_string_len(env, user_name.as_ptr(), user_name.len(),
        ErlNifCharEncoding::ERL_NIF_LATIN1) };
    let name_tuple = make_tuple(env, "pw_name", &name_str);
    let home_dir = user.home_dir().to_str().unwrap();
    let home_str = unsafe { enif_make_string_len(env, home_dir.as_ptr(), home_dir.len(),
        ErlNifCharEncoding::ERL_NIF_LATIN1) };
    let home_tuple = make_tuple(env, "pw_dir", &home_str);
    let shell = user.shell().to_str().unwrap();
    let shell_str = unsafe { enif_make_string_len(env, shell.as_ptr(), shell.len(),
        ErlNifCharEncoding::ERL_NIF_LATIN1) };
    let shell_tuple = make_tuple(env, "pw_shell", &shell_str);
    let list_elems = [uid_tuple, gid_tuple, name_tuple, home_tuple, shell_tuple];
    unsafe { enif_make_list_from_array(env, list_elems.as_ptr(), 5) }
}

/// Produce a 2-tuple consisting of 'ok' and the given result.
fn make_ok_result(env: *mut ErlNifEnv, result: *const ERL_NIF_TERM) -> ERL_NIF_TERM {
    make_tuple(env, "ok", result)
}

/// Produce a 2-tuple consisting of 'error' and the given reason.
fn make_err_result(env: *mut ErlNifEnv, reason: &str) -> ERL_NIF_TERM {
    let reason_str = unsafe { enif_make_string_len(env, reason.as_ptr(), reason.len(),
        ErlNifCharEncoding::ERL_NIF_LATIN1) };
    make_tuple(env, "error", &reason_str)
}

/// Produce a 2-tuple consisting of the label and the term.
/// The label is converted to an atom.
fn make_tuple(env: *mut ErlNifEnv, label: &str, result: *const ERL_NIF_TERM) -> ERL_NIF_TERM {
    let mut label_atom:ERL_NIF_TERM = unsafe { uninitialized() };
    let c_label_str = CString::new(label).unwrap();
    let c_label_nul = c_label_str.as_bytes_with_nul().as_ptr();
    // Try using an existing atom, but if that fails, create a new one.
    let atom_exists = unsafe { enif_make_existing_atom(
        env, c_label_nul, &mut label_atom, ErlNifCharEncoding::ERL_NIF_LATIN1) };
    if atom_exists == 0 {
        label_atom = unsafe { enif_make_atom(env, c_label_nul) };
    }
    let tuple_args = unsafe { [label_atom, *result] };
    unsafe { enif_make_tuple_from_array(env, tuple_args.as_ptr(), 2) }
}
