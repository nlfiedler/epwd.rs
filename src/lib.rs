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
fn getpwnam(name: String) -> Result<User, String> {
    match users::get_user_by_name(&name) {
        Some(user) => Ok(User::from(user)),
        None => Err(String::from("no such user"))
    }
}

#[rustler::nif]
fn getpwuid(uid: u32) -> Result<User, String> {
    match users::get_user_by_uid(uid as libc::uid_t) {
        Some(user) => Ok(User::from(user)),
        None => Err(String::from("no such user"))
    }
}

/// Produce a map consisting of the details of the given user.
/// Keys include pw_uid, pw_gid, pw_name, pw_dir, and pw_shell.
#[derive(NifMap)]
struct User {
    pub pw_uid: uid_t,
    pub pw_gid: gid_t,
    pub pw_name: String,
    pub pw_dir: String,
    pub pw_shell: String,
}

impl From<users::User> for User {
    fn from(user: users::User) -> User {
        User{
            pw_uid: user.uid(),
            pw_gid: user.primary_group_id(),
            pw_name: user.name().to_str().unwrap().to_string(),
            pw_dir: user.home_dir().to_str().unwrap().to_string(),
            pw_shell: user.shell().to_str().unwrap().to_string(),
        }
    }
}
