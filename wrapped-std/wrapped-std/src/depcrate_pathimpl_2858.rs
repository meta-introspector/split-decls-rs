// Generated macro for impl_2858 (impl)
macro_rules! Depcrate_pathimpl_2858 {
() => {
// Module: crate::path
// Provides: {"impl_2858"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < P : AsRef < Path > > Extend < P > for PathBuf { # [doc = " Extends `self` with [`Path`] elements from `iter`."] # [doc = ""] # [doc = " This uses [`push`](Self::push) to add each element, so can be used to adjoin multiple path"] # [doc = " [components](Components)."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " # use std::path::PathBuf;"] # [doc = " let mut path = PathBuf::from(\"/tmp\");"] # [doc = " path.extend([\"foo\", \"bar\", \"file.txt\"]);"] # [doc = " assert_eq!(path, PathBuf::from(\"/tmp/foo/bar/file.txt\"));"] # [doc = " ```"] # [doc = ""] # [doc = " See documentation for [`push`](Self::push) for more details on how the path is constructed."] fn extend < I : IntoIterator < Item = P > > (& mut self , iter : I) { iter . into_iter () . for_each (move | p | self . push (p . as_ref ())) ; } # [inline] fn extend_one (& mut self , p : P) { self . push (p . as_ref ()) ; } }
};
}
