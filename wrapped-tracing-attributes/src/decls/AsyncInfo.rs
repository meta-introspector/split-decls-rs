macro_rules! deps {
    () => {
        AsyncKind!();
    };
}

macro_rules! AsyncInfo {
    () => {
        deps!();
        pub (crate) struct AsyncInfo < 'block > { source_stmt : & 'block Stmt , kind : AsyncKind < 'block > , self_type : Option < TypePath > , input : & 'block ItemFn , }
    };
}

AsyncInfo!()