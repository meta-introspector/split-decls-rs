// Generated macro for impl_2848 (impl)
macro_rules! Depcrate_pathimpl_2848 {
() => {
// Module: crate::path
// Provides: {"impl_2848"}
// Dependencies: {}
# [stable (feature = "box_from_cow" , since = "1.45.0")] impl From < Cow < '_ , Path > > for Box < Path > { # [doc = " Creates a boxed [`Path`] from a clone-on-write pointer."] # [doc = ""] # [doc = " Converting from a `Cow::Owned` does not clone or allocate."] # [inline] fn from (cow : Cow < '_ , Path >) -> Box < Path > { match cow { Cow :: Borrowed (path) => Box :: from (path) , Cow :: Owned (path) => Box :: from (path) , } } }
};
}
