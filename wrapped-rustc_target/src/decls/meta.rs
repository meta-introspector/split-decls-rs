macro_rules! deps {
    () => {
        TargetMetadata!();
    };
}

macro_rules! meta {
    () => {
        deps!();
        pub (crate) fn meta () -> TargetMetadata { TargetMetadata { description : None , tier : Some (3) , host_tools : Some (false) , std : Some (true) } }
    };
}

meta!();