macro_rules! deps {
    () => {
        Accumulator!();
        Database!();
    };
}

macro_rules! prelude {
    () => {
        deps!();
        pub mod prelude { # [cfg (feature = "accumulator")] pub use crate :: accumulator :: Accumulator ; pub use crate :: { Database , Setter } ; }
    };
}

prelude!();