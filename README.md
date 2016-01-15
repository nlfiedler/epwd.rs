# epwd.rs

A basic [Erlang](http://www.erlang.org) interface to the POSIX user database (e.g. `getpwnam()` and friends), via [Rust](https://www.rust-lang.org) bindings. Currently very few fuctions are exposed, generally only those that are needed for the [tanuki](https://github.com/nlfiedler/tanuki) project.

## Requirements

* Erlang/OTP R17|R18
* [Rebar](https://github.com/rebar/rebar)
* Rust (1.3 or higher should work)
* Cargo

## Building and Testing

The following should clean and build everything from scratch, including downloading dependencies.

```
$ rebar clean
$ rebar compile
$ rebar ct
...
DONE.
Testing epwd.rs: TEST COMPLETE, 2 ok, 0 failed of 2 test cases
```

## Example

Include as a dependency in your release, using rebar...

```
{deps, [
    {epwd_rs, ".*", {git, "https://github.com/nlfiedler/epwd_rs", {tag, "0.1.0"}}}
]}.
```

Fetching the details of a user by their name...

```
Username = os:getenv("USER", "root"),
{ok, Results} = epwd_rs:getpwnam(Username),
Uid = proplists:lookup(pw_uid, Results),
io:format("User ~p ID: ~p~n", [Username, Uid]).
```
