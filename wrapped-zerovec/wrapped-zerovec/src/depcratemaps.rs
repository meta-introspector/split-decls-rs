// Generated macro for maps (module)
macro_rules! Depcratemaps {
() => {
// Module: crate
// Provides: {"maps"}
// Dependencies: {}
# [cfg (feature = "alloc")] pub mod maps { # ! [doc = " This module contains additional utility types and traits for working with"] # ! [doc = " [`ZeroMap`] and [`ZeroMap2d`]. See their docs for more details on the general purpose"] # ! [doc = " of these types."] # ! [doc = ""] # ! [doc = " [`ZeroMapBorrowed`] and [`ZeroMap2dBorrowed`] are versions of [`ZeroMap`] and [`ZeroMap2d`]"] # ! [doc = " that can be used when you wish to guarantee that the map data is always borrowed, leading to"] # ! [doc = " relaxed lifetime constraints."] # ! [doc = ""] # ! [doc = " The [`ZeroMapKV`] trait is required to be implemented on any type that needs to be used"] # ! [doc = " within a map type. [`ZeroVecLike`] and [`MutableZeroVecLike`] are traits used in the"] # ! [doc = " internal workings of the map types, and should typically not be used or implemented by"] # ! [doc = " users of this crate."] # [doc (no_inline)] pub use crate :: map :: ZeroMap ; pub use crate :: map :: ZeroMapBorrowed ; # [doc (no_inline)] pub use crate :: map2d :: ZeroMap2d ; pub use crate :: map2d :: ZeroMap2dBorrowed ; pub use crate :: map :: { MutableZeroVecLike , ZeroMapKV , ZeroVecLike } ; pub use crate :: map2d :: ZeroMap2dCursor ; }
};
}
