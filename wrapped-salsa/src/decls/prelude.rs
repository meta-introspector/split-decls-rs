macro_rules! prelude {
    () => {
        pub mod prelude { # [cfg (feature = "accumulator")] pub use crate :: accumulator :: Accumulator ; pub use crate :: { Database , Setter } ; }
    };
}

prelude!()