macro_rules! deps {
    () => {
        Metadata!();
        Error!();
        Result!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl Display for Error { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { use self :: Error :: * ; match self { Cargo (e) => write ! (f , "failed to execute cargo: {}" , e) , CargoFail => write ! (f , "cargo reported an error") , GetManifest (path , e) => write ! (f , "failed to read manifest {}: {}" , path . display () , e) , Glob (e) => write ! (f , "{}" , e) , Io (e) => write ! (f , "{}" , e) , Metadata (e) => write ! (f , "failed to read cargo metadata: {}" , e) , Mismatch => write ! (f , "compiler error does not match expected error") , NoWorkspaceManifest => write ! (f , "Cargo.toml uses edition.workspace=true, but no edition found in workspace's manifest") , Open (path , e) => write ! (f , "{}: {}" , path . display () , e) , Pattern (e) => write ! (f , "{}" , e) , ProjectDir => write ! (f , "failed to determine name of project dir") , ReadStderr (e) => write ! (f , "failed to read stderr file: {}" , e) , RunFailed => write ! (f , "execution of the test case was unsuccessful") , ShouldNotHaveCompiled => { write ! (f , "expected test case to fail to compile, but it succeeded") } TomlDe (e) => write ! (f , "{}" , e) , TomlSer (e) => write ! (f , "{}" , e) , UpdateVar (var) => write ! (f , "unrecognized value of TRYBUILD: {:?}" , var . to_string_lossy () ,) , WriteStderr (e) => write ! (f , "failed to write stderr file: {}" , e) , } } }
    };
}

impl_70!();