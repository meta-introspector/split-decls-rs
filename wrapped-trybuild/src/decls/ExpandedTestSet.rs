macro_rules! deps {
    () => {
        ExpandedTest!();
    };
}

macro_rules! ExpandedTestSet {
    () => {
        deps!();
        struct ExpandedTestSet { vec : Vec < ExpandedTest > , path_to_index : Map < PathBuf , usize > , }
    };
}

ExpandedTestSet!();