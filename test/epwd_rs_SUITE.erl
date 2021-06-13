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

% So this has to be defined now?
end_per_suite(Config) ->
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
    Username = list_to_binary(os:getenv("USER", "root")),
    {ok, Results} = epwd_rs:getpwnam(Username),
    ?assert(is_map(Results)),
    ?assertEqual(5, maps:size(Results)),
    ?assert(is_integer(maps:get(pw_uid, Results))),
    ?assert(is_integer(maps:get(pw_gid, Results))),
    ?assertEqual(Username, maps:get(pw_name, Results)),
    ?assert(is_binary(maps:get(pw_dir, Results))),
    ?assert(is_binary(maps:get(pw_shell, Results))),
    %
    % Negative case, no such user
    %
    {error, Reason} = epwd_rs:getpwnam(<<"bozo">>),
    ?assertEqual(<<"no such user">>, Reason),
    ok.

test_getpwuid(_Config) ->
    %
    % Fetch a user by an identifier that we are pretty sure exists on most
    % systems, and very likely has the name "root".
    %
    {ok, Results} = epwd_rs:getpwuid(0),
    ?assert(is_map(Results)),
    ?assertEqual(5, maps:size(Results)),
    ?assertEqual(0, maps:get(pw_uid, Results)),
    ?assert(is_integer(maps:get(pw_gid, Results))),
    ?assertEqual(<<"root">>, maps:get(pw_name, Results)),
    ?assert(is_binary(maps:get(pw_dir, Results))),
    ?assert(is_binary(maps:get(pw_shell, Results))),
    ok.
