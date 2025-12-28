macro_rules! deps {
    () => {
        Directory!();
        PathDependency!();
    };
}

macro_rules! Context {
    () => {
        deps!();
        # [derive (Copy , Clone)] pub (crate) struct Context < 'a > { pub krate : & 'a str , pub source_dir : & 'a Directory , pub workspace : & 'a Directory , pub input_file : & 'a Path , pub target_dir : & 'a Directory , pub path_dependencies : & 'a [PathDependency] , }
    };
}

Context!()