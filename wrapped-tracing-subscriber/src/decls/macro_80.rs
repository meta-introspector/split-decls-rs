macro_rules! macro_80 {
    () => {
        feature ! { #! [all (feature = "fmt" , feature = "std")] pub use crate :: fmt :: writer :: MakeWriterExt as _ ; }
    };
}

macro_80!()