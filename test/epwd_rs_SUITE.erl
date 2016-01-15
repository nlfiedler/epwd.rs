%% -*- coding: utf-8 -*-
%% -------------------------------------------------------------------
%%
%% Copyright (c) 2016 Nathan Fiedler
%%
%% This file is provided to you under the Apache License,
%% Version 2.0 (the "License"); you may not use this file
%% except in compliance with the License. You may obtain
%% a copy of the License at
%%
%% http://www.apache.org/licenses/LICENSE-2.0
%%
%% Unless required by applicable law or agreed to in writing,
%% software distributed under the License is distributed on an
%% "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
%% KIND, either express or implied. See the License for the
%% specific language governing permissions and limitations
%% under the License.
%%
%% -------------------------------------------------------------------

-module(epwd_rs_SUITE).
-compile(export_all).

-include_lib("common_test/include/ct.hrl").
-include_lib("eunit/include/eunit.hrl").

init_per_suite(Config) ->
    ok = application:load(epwd_rs),
    Config.

all() ->
    [
        test_getpwnam,
        test_getpwuid
    ].

test_getpwnam(_Config) ->
    %
    % Fetch a user we know will exist (the current user, or root) and verify
    % that the returned structure appears to be correct. Most values cannot be
    % asserted as they are system dependent.
    %
    Username = os:getenv("USER", "root"),
    {ok, Results} = epwd_rs:getpwnam(Username),
    ?assert(is_list(Results)),
    ?assertEqual(5, length(Results)),
    ?assert(is_tuple(proplists:lookup(pw_uid, Results))),
    ?assert(is_tuple(proplists:lookup(pw_gid, Results))),
    ?assertEqual(Username, proplists:get_value(pw_name, Results)),
    ?assert(is_tuple(proplists:lookup(pw_dir, Results))),
    ?assert(is_tuple(proplists:lookup(pw_shell, Results))),
    %
    % Negative case, no such user
    %
    {error, Reason} = epwd_rs:getpwnam("bozo"),
    ?assertEqual("no such user", Reason),
    ok.

test_getpwuid(_Config) ->
    %
    % Fetch a user by an identifier that we are pretty sure exists on most
    % systems, and very likely has the name "root".
    %
    {ok, Results} = epwd_rs:getpwuid(0),
    ?assert(is_list(Results)),
    ?assertEqual(5, length(Results)),
    ?assertEqual(0, proplists:get_value(pw_uid, Results)),
    ?assert(is_tuple(proplists:lookup(pw_gid, Results))),
    ?assertEqual("root", proplists:get_value(pw_name, Results)),
    ?assert(is_tuple(proplists:lookup(pw_dir, Results))),
    ?assert(is_tuple(proplists:lookup(pw_shell, Results))),
    ok.
