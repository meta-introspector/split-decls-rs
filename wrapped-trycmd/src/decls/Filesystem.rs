macro_rules! deps {
    () => {
        FileStatus!();
    };
}

macro_rules! Filesystem {
    () => {
        deps!();
        # [derive (Clone , Default , Debug , PartialEq , Eq)] struct Filesystem { context : Vec < FileStatus > , }
    };
}

Filesystem!();