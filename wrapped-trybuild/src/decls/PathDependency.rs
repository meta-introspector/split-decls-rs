macro_rules! deps {
    () => {
        Directory!();
    };
}

macro_rules! PathDependency {
    () => {
        deps!();
        # [derive (Debug)] pub (crate) struct PathDependency { pub name : String , pub normalized_path : Directory , }
    };
}

PathDependency!();