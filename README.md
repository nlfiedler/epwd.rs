# epwd.rs

A basic [Erlang](http://www.erlang.org) interface to the POSIX user database (e.g. `getpwnam()` and friends), via [Rust](https://www.rust-lang.org) bindings. Currently only two fuctions are exposed, the ones that are needed for the [tanuki](https://github.com/nlfiedler/tanuki) project.

## Requirements

* Erlang/OTP R17 or higher
* [Rebar3](http://www.rebar3.org/) 3.0.0 or higher
* Rust (1.11 or higher should work)
* Cargo

## Building and Testing

The following should build and test everything:

```
$ rebar3 ct
...
===> Running Common Test suites...
%%% epwd_rs_SUITE ==> test_getpwnam: OK
%%% epwd_rs_SUITE ==> test_getpwuid: OK
All 2 tests passed.
```

## Example

Include as a dependency in your release, using rebar3...

```
{deps, [
    {epwd_rs, {git, "https://github.com/nlfiedler/epwd_rs", {tag, "0.1.8"}}}
]}.
```

Be sure to include `epwd_rs` in the `included_applications` list of your application configuration before building a release.

Fetching the details of a user by their name...

```
Username = os:getenv("USER", "root"),
{ok, Results} = epwd_rs:getpwnam(Username),
Uid = proplists:lookup(pw_uid, Results),
io:format("User ~p ID: ~p~n", [Username, Uid]).
```
