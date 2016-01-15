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
-module(epwd_rs).
-export([getpwnam/1, getpwuid/1]).
-on_load(init/0).

-define(APPNAME, epwd_rs).
-define(LIBNAME, libepwd_rs).

init() ->
    % tried using code:priv_dir/1 but it never seemed to work
    PrivDir = filename:join([filename:dirname(code:which(?APPNAME)), "..", "priv"]),
    SoName = filename:join(PrivDir, ?LIBNAME),
    ok = erlang:load_nif(SoName, 0).

%
% @doc Retrieves the details for the named user as a property list.
%      Returns {ok, Details} if successful, and {error, Reason} otherwise.
%      The property list keys are: pw_uid, pw_gid, pw_name, pw_dir, pw_shell
%
getpwnam(_Username) ->
    exit(nif_library_not_loaded).

%
% @doc Retrieves the details for the identified user as a property list.
%      Returns {ok, Details} if successful, and {error, Reason} otherwise.
%      The property list keys are: pw_uid, pw_gid, pw_name, pw_dir, pw_shell
%
getpwuid(_Uid) ->
    exit(nif_library_not_loaded).
