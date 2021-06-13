# Change Log

All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - 2021-06-12
### Changed
- Migrate from erlang_nif-sys to Rustler
- Return map objects instead of property lists
- Accept and return binary strings instead of char lists

## [0.1.9] - 2017-07-03
### Changed
- Upgrade erlang_nif-sys to support Erlang/OTP 20.

## [0.1.8] - 2017-01-25
### Changed
- Upgrade to erlang_nif-sys 0.5.4
- Changed `crate-type` to `cdylib`

## [0.1.7] - 2016-07-18
### Changed
- Upgrade to erlang_nif-sys (née ruster_unsafe) 0.5.2

## [0.1.6] - 2016-03-26
### Changed
- Upgrade to rust-users 0.5.1

## [0.1.5] - 2016-03-23
### Changed
- Switch back to original ruster_unsafe crate.

## [0.1.4] - 2016-03-17
### Changed
- Allow libc version 0.2 or higher
- Upgrade ruster_unsafe to latest release

## [0.1.3] - 2016-03-14
### Changed
- Upgrade to rebar3 build system.
- Enhanced the native code loading function.

## [0.1.2] - 2016-01-25
### Changed
- Fix makefile so it copies library on non-Darwin systems, too.

## [0.1.1] - 2016-01-25
### Changed
- Use GNU make when available to build the Rust code.

## [0.1.0] - 2016-01-14
### Changed
- Initial release
