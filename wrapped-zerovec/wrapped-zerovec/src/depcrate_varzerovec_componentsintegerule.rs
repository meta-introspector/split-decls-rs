// Generated macro for IntegerULE (trait)
macro_rules! Depcrate_varzerovec_componentsIntegerULE {
() => {
// Module: crate::varzerovec::components
// Provides: {"IntegerULE"}
// Dependencies: {}
# [doc = " This trait represents various ULE types that can be used to represent an integer"] # [doc = ""] # [doc = " Do not implement this trait, its internals may be changed in the future,"] # [doc = " and all of its associated items are hidden from the docs."] # [doc (hidden)] pub unsafe trait IntegerULE : ULE { # [doc = " The error to show when unable to construct a vec"] # [doc (hidden)] const TOO_LARGE_ERROR : & 'static str ; # [doc = " Safety: must be sizeof(self)"] # [doc (hidden)] const SIZE : usize ; # [doc = " Safety: must be maximum integral value represented here"] # [doc (hidden)] const MAX_VALUE : u32 ; # [doc = " Safety: Must roundtrip with from_usize and represent the correct"] # [doc = " integral value"] # [doc (hidden)] fn iule_to_usize (self) -> usize ; # [doc (hidden)] fn iule_from_usize (x : usize) -> Option < Self > ; # [doc = " Safety: Should always convert a buffer into an array of Self with the correct length"] # [doc (hidden)] # [cfg (feature = "alloc")] fn iule_from_bytes_unchecked_mut (bytes : & mut [u8]) -> & mut [Self] ; }
};
}
