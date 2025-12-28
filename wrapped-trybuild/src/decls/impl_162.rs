macro_rules! deps {
    () => {
        Result!();
        Variations!();
        Project!();
        Update!();
        Name!();
        Stderr!();
        CanonicalPath!();
        Expected!();
        Outcome!();
        Test!();
        Error!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl Test { fn run (& self , project : & Project , name : & Name) -> Result < Outcome > { let show_expected = project . has_pass && project . has_compile_fail ; message :: begin_test (self , show_expected) ; check_exists (& self . path) ? ; let mut path_map = Map :: new () ; let src_path = CanonicalPath :: new (& project . source_dir . join (& self . path)) ; path_map . insert (src_path . clone () , (name , self)) ; let output = cargo :: build_test (project , name) ? ; let parsed = parse_cargo_json (project , & output . stdout , & path_map) ; let fallback = Stderr :: default () ; let this_test = parsed . stderrs . get (& src_path) . unwrap_or (& fallback) ; self . check (project , name , this_test , & parsed . stdout) } fn check (& self , project : & Project , name : & Name , result : & Stderr , build_stdout : & str ,) -> Result < Outcome > { let check = match self . expected { Expected :: Pass => Test :: check_pass , Expected :: CompileFail => Test :: check_compile_fail , } ; check (self , project , name , result . success , build_stdout , & result . stderr ,) } fn check_pass (& self , project : & Project , name : & Name , success : bool , build_stdout : & str , variations : & Variations ,) -> Result < Outcome > { let preferred = variations . preferred () ; if ! success { message :: failed_to_build (preferred) ; return Err (Error :: CargoFail) ; } let mut output = cargo :: run_test (project , name) ? ; output . stdout . splice (.. 0 , build_stdout . bytes ()) ; message :: output (preferred , & output) ; if output . status . success () { Ok (Outcome :: Passed) } else { Err (Error :: RunFailed) } } fn check_compile_fail (& self , project : & Project , _name : & Name , success : bool , build_stdout : & str , variations : & Variations ,) -> Result < Outcome > { let preferred = variations . preferred () ; if success { message :: should_not_have_compiled () ; message :: fail_output (Fail , build_stdout) ; message :: warnings (preferred) ; return Err (Error :: ShouldNotHaveCompiled) ; } let stderr_path = self . path . with_extension ("stderr") ; if ! stderr_path . exists () { let outcome = match project . update { Update :: Wip => { let wip_dir = Path :: new ("wip") ; fs :: create_dir_all (wip_dir) ? ; let gitignore_path = wip_dir . join (".gitignore") ; fs :: write (gitignore_path , "*\n") ? ; let stderr_name = stderr_path . file_name () . unwrap_or_else (| | OsStr :: new ("test.stderr")) ; let wip_path = wip_dir . join (stderr_name) ; message :: write_stderr_wip (& wip_path , & stderr_path , preferred) ; fs :: write (wip_path , preferred) . map_err (Error :: WriteStderr) ? ; Outcome :: CreatedWip } Update :: Overwrite => { message :: overwrite_stderr (& stderr_path , preferred) ; fs :: write (stderr_path , preferred) . map_err (Error :: WriteStderr) ? ; Outcome :: Passed } } ; message :: fail_output (Warn , build_stdout) ; return Ok (outcome) ; } let expected = fs :: read_to_string (& stderr_path) . map_err (Error :: ReadStderr) ? . replace ("\r\n" , "\n") ; if variations . any (| stderr | expected == stderr) { message :: ok () ; return Ok (Outcome :: Passed) ; } match project . update { Update :: Wip => { message :: mismatch (& expected , preferred) ; Err (Error :: Mismatch) } Update :: Overwrite => { message :: overwrite_stderr (& stderr_path , preferred) ; fs :: write (stderr_path , preferred) . map_err (Error :: WriteStderr) ? ; Ok (Outcome :: Passed) } } } }
    };
}

impl_162!();