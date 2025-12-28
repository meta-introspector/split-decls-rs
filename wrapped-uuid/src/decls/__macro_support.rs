macro_rules! __macro_support {
    () => {
        # [doc (hidden)] pub mod __macro_support { pub use crate :: std :: result :: Result :: { Err , Ok } ; }
    };
}

__macro_support!();