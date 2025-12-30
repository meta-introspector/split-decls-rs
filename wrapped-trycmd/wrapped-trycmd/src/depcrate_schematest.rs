// Generated macro for test (module)
macro_rules! Depcrate_schematest {
() => {
// Module: crate::schema
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn parse_trycmd_empty () { let expected = TryCmd { steps : vec ! [] , .. Default :: default () } ; let actual = TryCmd :: parse_trycmd ("") . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_trycmd_empty_fence () { let expected = TryCmd { steps : vec ! [] , .. Default :: default () } ; let actual = TryCmd :: parse_trycmd ("
```
```
" ,) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_trycmd_command () { let expected = TryCmd { steps : vec ! [Step { id : Some ("3" . into ()) , bin : Some (Bin :: Name ("cmd" . into ())) , expected_status : Some (CommandStatus :: Success) , stderr_to_stdout : true , expected_stdout_source : Some (4 .. 4) , expected_stdout : Some (crate :: Data :: new ()) , expected_stderr : None , .. Default :: default () }] , .. Default :: default () } ; let actual = TryCmd :: parse_trycmd ("
```
$ cmd
```
" ,) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_trycmd_command_line () { let expected = TryCmd { steps : vec ! [Step { id : Some ("3" . into ()) , bin : Some (Bin :: Name ("cmd" . into ())) , args : vec ! ["arg1" . into () , "arg with space" . into ()] , expected_status : Some (CommandStatus :: Success) , stderr_to_stdout : true , expected_stdout_source : Some (4 .. 4) , expected_stdout : Some (crate :: Data :: new ()) , expected_stderr : None , .. Default :: default () }] , .. Default :: default () } ; let actual = TryCmd :: parse_trycmd ("
```
$ cmd arg1 'arg with space'
```
" ,) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_trycmd_multi_line () { let expected = TryCmd { steps : vec ! [Step { id : Some ("3" . into ()) , bin : Some (Bin :: Name ("cmd" . into ())) , args : vec ! ["arg1" . into () , "arg with space" . into ()] , expected_status : Some (CommandStatus :: Success) , stderr_to_stdout : true , expected_stdout_source : Some (5 .. 5) , expected_stdout : Some (crate :: Data :: new ()) , expected_stderr : None , .. Default :: default () }] , .. Default :: default () } ; let actual = TryCmd :: parse_trycmd ("
```
$ cmd arg1
> 'arg with space'
```
" ,) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_trycmd_env () { let expected = TryCmd { steps : vec ! [Step { id : Some ("3" . into ()) , bin : Some (Bin :: Name ("cmd" . into ())) , env : Env { add : IntoIterator :: into_iter ([("KEY1" . into () , "VALUE1" . into ()) , ("KEY2" . into () , "VALUE2 with space" . into ()) ,]) . collect () , .. Default :: default () } , expected_status : Some (CommandStatus :: Success) , stderr_to_stdout : true , expected_stdout_source : Some (4 .. 4) , expected_stdout : Some (crate :: Data :: new ()) , expected_stderr : None , .. Default :: default () }] , .. Default :: default () } ; let actual = TryCmd :: parse_trycmd ("
```
$ KEY1=VALUE1 KEY2='VALUE2 with space' cmd
```
" ,) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_trycmd_status () { let expected = TryCmd { steps : vec ! [Step { id : Some ("3" . into ()) , bin : Some (Bin :: Name ("cmd" . into ())) , expected_status_source : Some (4) , expected_status : Some (CommandStatus :: Skipped) , stderr_to_stdout : true , expected_stdout_source : Some (5 .. 5) , expected_stdout : Some (crate :: Data :: new ()) , expected_stderr : None , .. Default :: default () }] , .. Default :: default () } ; let actual = TryCmd :: parse_trycmd ("
```
$ cmd
? skipped
```
" ,) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_trycmd_status_code () { let expected = TryCmd { steps : vec ! [Step { id : Some ("3" . into ()) , bin : Some (Bin :: Name ("cmd" . into ())) , expected_status_source : Some (4) , expected_status : Some (CommandStatus :: Code (- 1)) , stderr_to_stdout : true , expected_stdout_source : Some (5 .. 5) , expected_stdout : Some (crate :: Data :: new ()) , expected_stderr : None , .. Default :: default () }] , .. Default :: default () } ; let actual = TryCmd :: parse_trycmd ("
```
$ cmd
? -1
```
" ,) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_trycmd_stdout () { let expected = TryCmd { steps : vec ! [Step { id : Some ("3" . into ()) , bin : Some (Bin :: Name ("cmd" . into ())) , expected_status : Some (CommandStatus :: Success) , stderr_to_stdout : true , expected_stdout_source : Some (4 .. 6) , expected_stdout : Some (crate :: Data :: text ("Hello World\n")) , expected_stderr : None , .. Default :: default () }] , .. Default :: default () } ; let actual = TryCmd :: parse_trycmd ("
```
$ cmd
Hello World

```" ,) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_trycmd_escaped_stdout () { let expected = TryCmd { steps : vec ! [Step { id : Some ("3" . into ()) , bin : Some (Bin :: Name ("cmd" . into ())) , expected_status : Some (CommandStatus :: Success) , stderr_to_stdout : true , expected_stdout_source : Some (4 .. 7) , expected_stdout : Some (crate :: Data :: text ("```\nHello World\n```")) , expected_stderr : None , .. Default :: default () }] , .. Default :: default () } ; let actual = TryCmd :: parse_trycmd ("
````
$ cmd
```
Hello World
```
````" ,) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_trycmd_multi_step () { let expected = TryCmd { steps : vec ! [Step { id : Some ("3" . into ()) , bin : Some (Bin :: Name ("cmd1" . into ())) , expected_status_source : Some (4) , expected_status : Some (CommandStatus :: Code (1)) , stderr_to_stdout : true , expected_stdout_source : Some (5 .. 5) , expected_stdout : Some (crate :: Data :: new ()) , expected_stderr : None , .. Default :: default () } , Step { id : Some ("5" . into ()) , bin : Some (Bin :: Name ("cmd2" . into ())) , expected_status : Some (CommandStatus :: Success) , stderr_to_stdout : true , expected_stdout_source : Some (6 .. 6) , expected_stdout : Some (crate :: Data :: new ()) , expected_stderr : None , .. Default :: default () } ,] , .. Default :: default () } ; let actual = TryCmd :: parse_trycmd ("
```
$ cmd1
? 1
$ cmd2
```
" ,) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_trycmd_info_string () { let expected = TryCmd { steps : vec ! [Step { id : Some ("3" . into ()) , bin : Some (Bin :: Name ("bare-cmd" . into ())) , expected_status_source : Some (4) , expected_status : Some (CommandStatus :: Code (1)) , stderr_to_stdout : true , expected_stdout_source : Some (5 .. 5) , expected_stdout : Some (crate :: Data :: new ()) , expected_stderr : None , .. Default :: default () } , Step { id : Some ("8" . into ()) , bin : Some (Bin :: Name ("trycmd-cmd" . into ())) , expected_status_source : Some (9) , expected_status : Some (CommandStatus :: Code (1)) , stderr_to_stdout : true , expected_stdout_source : Some (10 .. 10) , expected_stdout : Some (crate :: Data :: new ()) , expected_stderr : None , .. Default :: default () } , Step { id : Some ("18" . into ()) , bin : Some (Bin :: Name ("console-cmd" . into ())) , expected_status_source : Some (19) , expected_status : Some (CommandStatus :: Code (1)) , stderr_to_stdout : true , expected_stdout_source : Some (20 .. 20) , expected_stdout : Some (crate :: Data :: new ()) , expected_stderr : None , .. Default :: default () } ,] , .. Default :: default () } ; let actual = TryCmd :: parse_trycmd ("
```
$ bare-cmd
? 1
```

```trycmd
$ trycmd-cmd
? 1
```

```sh
$ sh-cmd
? 1
```

```console
$ console-cmd
? 1
```

```ignore
$ rust-cmd1
? 1
```

```trycmd,ignore
$ rust-cmd1
? 1
```

```rust
$ rust-cmd1
? 1
```
" ,) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_toml_minimal () { let expected = OneShot { .. Default :: default () } ; let actual = OneShot :: parse_toml ("") . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_toml_minimal_env () { let expected = OneShot { .. Default :: default () } ; let actual = OneShot :: parse_toml ("[env]") . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_toml_bin_name () { let expected = OneShot { bin : Some (Bin :: Name ("cmd" . into ())) , .. Default :: default () } ; let actual = OneShot :: parse_toml ("bin.name = 'cmd'") . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_toml_bin_path () { let expected = OneShot { bin : Some (Bin :: Path ("/usr/bin/cmd" . into ())) , .. Default :: default () } ; let actual = OneShot :: parse_toml ("bin.path = '/usr/bin/cmd'") . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_toml_args_split () { let expected = OneShot { args : Args :: Split (vec ! ["arg1" . into () , "arg with space" . into ()]) , .. Default :: default () } ; let actual = OneShot :: parse_toml (r#"args = ["arg1", "arg with space"]"#) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_toml_args_joined () { let expected = OneShot { args : Args :: Joined (JoinedArgs :: from_vec (vec ! ["arg1" . into () , "arg with space" . into () ,])) , .. Default :: default () } ; let actual = OneShot :: parse_toml (r#"args = "arg1 'arg with space'""#) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_toml_status_success () { let expected = OneShot { status : Some (CommandStatus :: Success) , .. Default :: default () } ; let actual = OneShot :: parse_toml ("status = 'success'") . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn parse_toml_status_code () { let expected = OneShot { status : Some (CommandStatus :: Code (42)) , .. Default :: default () } ; let actual = OneShot :: parse_toml ("status.code = 42") . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn replace_lines_same_line_count () { let input = "One\nTwo\nThree" ; let line_nums = 2 .. 3 ; let replacement = "World\n" ; let expected = "One\nWorld\nThree" ; let mut actual = input . to_owned () ; replace_lines (& mut actual , line_nums , replacement) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn replace_lines_grow () { let input = "One\nTwo\nThree" ; let line_nums = 2 .. 3 ; let replacement = "World\nTrees\n" ; let expected = "One\nWorld\nTrees\nThree" ; let mut actual = input . to_owned () ; replace_lines (& mut actual , line_nums , replacement) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn replace_lines_shrink () { let input = "One\nTwo\nThree" ; let line_nums = 2 .. 3 ; let replacement = "" ; let expected = "One\nThree" ; let mut actual = input . to_owned () ; replace_lines (& mut actual , line_nums , replacement) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn replace_lines_no_trailing () { let input = "One\nTwo\nThree" ; let line_nums = 2 .. 3 ; let replacement = "World" ; let expected = "One\nWorld\nThree" ; let mut actual = input . to_owned () ; replace_lines (& mut actual , line_nums , replacement) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn replace_lines_empty_range () { let input = "One\nTwo\nThree" ; let line_nums = 2 .. 2 ; let replacement = "World\n" ; let expected = "One\nWorld\nTwo\nThree" ; let mut actual = input . to_owned () ; replace_lines (& mut actual , line_nums , replacement) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn overwrite_toml_status_success () { let expected = r#"
bin.name = "cmd"
"# ; let actual = overwrite_toml_status (exit_code_to_status (0) , r#"
bin.name = "cmd"
status = "failed"
"# . into () ,) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn overwrite_toml_status_failed () { let expected = r#"
bin.name = "cmd"
status.code = 1
"# ; let actual = overwrite_toml_status (exit_code_to_status (1) , r#"
bin.name = "cmd"
"# . into () ,) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn overwrite_toml_status_keeps_style () { let expected = r#"
bin.name = "cmd"
status = { code = 1 } # comment
"# ; let actual = overwrite_toml_status (exit_code_to_status (1) , r#"
bin.name = "cmd"
status = { code = 2 } # comment
"# . into () ,) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn overwrite_trycmd_status_success () { let expected = r#"
```
$ cmd arg
foo
bar
```
"# ; let mut actual = r"
```
$ cmd arg
? failed
foo
bar
```
" . to_owned () ; let step = & TryCmd :: parse_trycmd (& actual) . unwrap () . steps [0] ; overwrite_trycmd_status (Some (exit_code_to_status (0)) , step , & mut step . expected_stdout_source . clone () . unwrap () , & mut actual ,) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn overwrite_trycmd_status_failed () { let expected = r#"
```
$ cmd arg
? 1
foo
bar
```
"# ; let mut actual = r"
```
$ cmd arg
? 2
foo
bar
```
" . to_owned () ; let step = & TryCmd :: parse_trycmd (& actual) . unwrap () . steps [0] ; overwrite_trycmd_status (Some (exit_code_to_status (1)) , step , & mut step . expected_stdout_source . clone () . unwrap () , & mut actual ,) . unwrap () ; assert_eq ! (expected , actual) ; } # [test] fn overwrite_trycmd_status_keeps_style () { let expected = r#"
```
$ cmd arg
? success
foo
bar
```
"# ; let mut actual = r"
```
$ cmd arg
? success
foo
bar
```
" . to_owned () ; let step = & TryCmd :: parse_trycmd (& actual) . unwrap () . steps [0] ; overwrite_trycmd_status (Some (exit_code_to_status (0)) , step , & mut step . expected_stdout_source . clone () . unwrap () , & mut actual ,) . unwrap () ; assert_eq ! (expected , actual) ; } # [cfg (unix)] fn exit_code_to_status (code : u8) -> std :: process :: ExitStatus { use std :: os :: unix :: process :: ExitStatusExt ; std :: process :: ExitStatus :: from_raw ((code as i32) << 8) } # [cfg (windows)] fn exit_code_to_status (code : u8) -> std :: process :: ExitStatus { use std :: os :: windows :: process :: ExitStatusExt ; std :: process :: ExitStatus :: from_raw (code as u32) } # [test] fn exit_code_to_status_works () { assert_eq ! (exit_code_to_status (42) . code () , Some (42)) ; } }
};
}
