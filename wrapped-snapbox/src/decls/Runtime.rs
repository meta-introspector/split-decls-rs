macro_rules! deps {
    () => {
        SourceFileRuntime!();
        PathRuntime!();
    };
}

macro_rules! Runtime {
    () => {
        deps!();
        # [derive (Default)] pub (crate) struct Runtime { per_file : Vec < SourceFileRuntime > , path_count : Vec < PathRuntime > , }
    };
}

Runtime!()