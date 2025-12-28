macro_rules! deps {
    () => {
        Result!();
        ArgsInHash!();
        Creation!();
        DirectoryRoot!();
    };
}

macro_rules! scripted_fixture_writable_with_args_inner {
    () => {
        deps!();
        fn scripted_fixture_writable_with_args_inner (script_name : impl AsRef < Path > , args : impl IntoIterator < Item = impl Into < String > > , mode : Creation , root : DirectoryRoot , args_in_hash : ArgsInHash ,) -> Result < tempfile :: TempDir > { let dst = tempfile :: TempDir :: new () ? ; Ok (match mode { Creation :: CopyFromReadOnly => { let ro_dir = scripted_fixture_read_only_with_args_inner (script_name , args , None , root , args_in_hash) ? ; copy_recursively_into_existing_dir (ro_dir , dst . path ()) ? ; dst } Creation :: ExecuteScript => { scripted_fixture_read_only_with_args_inner (script_name , args , dst . path () . into () , root , args_in_hash) ? ; dst } }) }
    };
}

scripted_fixture_writable_with_args_inner!()