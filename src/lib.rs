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

extern crate rustler;
extern crate users;
extern crate libc;

use rustler::NifMap;
use users::os::unix::UserExt;
use libc::{uid_t, gid_t};

// Create NIF module data and init function.
rustler::init!("epwd_rs", [getpwnam, getpwuid]);

/// Retrieve the details for the named user as a property list.
#[rustler::nif]
fn getpwnam(name: Vec<u8>) -> Result<User, Vec<u8>> {
   let rname = std::str::from_utf8(&name);
   if rname.is_err() {
       return Err("invalid name".as_bytes().to_vec());
   }

    match users::get_user_by_name(&rname.unwrap()) {
        Some(user) => Ok(User::from(user)),
        None => Err("no such user".as_bytes().to_vec())
    }
}

#[rustler::nif]
fn getpwuid(uid: u32) -> Result<User, Vec<u8>> {
    match users::get_user_by_uid(uid as libc::uid_t) {
        Some(user) => Ok(User::from(user)),
        None => Err("no such user".as_bytes().to_vec())
    }
}

/// Produce a map consisting of the details of the given user.
/// Keys include pw_uid, pw_gid, pw_name, pw_dir, and pw_shell.
#[derive(NifMap)]
struct User {
    pub pw_uid: uid_t,
    pub pw_gid: gid_t,
    pub pw_name: Vec<u8>,
    pub pw_dir: Vec<u8>,
    pub pw_shell: Vec<u8>,
}

impl From<users::User> for User {
    fn from(user: users::User) -> User {
        User{
            pw_uid: user.uid(),
            pw_gid: user.primary_group_id(),
            pw_name: user.name().to_str().unwrap().as_bytes().to_vec(),
            pw_dir: user.home_dir().to_str().unwrap().as_bytes().to_vec(),
            pw_shell: user.shell().to_str().unwrap().as_bytes().to_vec(),
        }
    }
}
