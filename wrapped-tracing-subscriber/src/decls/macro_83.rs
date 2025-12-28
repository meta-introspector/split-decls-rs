macro_rules! macro_83 {
    () => {
        feature ! { #! [all (feature = "registry" , feature = "std")] mod sharded ; mod stack ; pub use sharded :: Data ; pub use sharded :: Registry ; use crate :: filter :: FilterId ; }
    };
}

macro_83!();