macro_rules! deps {
    () => {
        Manifest!();
        Directory!();
        Update!();
        PathDependency!();
    };
}

macro_rules! Project {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct Project { pub dir : Directory , source_dir : Directory , pub target_dir : Directory , pub name : String , update : Update , pub has_pass : bool , has_compile_fail : bool , pub features : Option < Vec < String > > , pub workspace : Directory , pub path_dependencies : Vec < PathDependency > , manifest : Manifest , pub keep_going : bool , }
    };
}

Project!();