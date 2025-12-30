// Generated macro for alloc_support (module)
macro_rules! Depcrate_dataalloc_support {
() => {
// Module: crate::data
// Provides: {"alloc_support"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod alloc_support { use super :: * ; use crate :: std :: { borrow :: ToOwned , string :: String } ; impl < 'computed > Clone for Label < 'computed > { fn clone (& self) -> Self { if let Some (owned) = self . backing_field_owned { Label :: new_owned (unsafe { & * owned } . to_owned ()) } else { Label { value_computed : self . value_computed , backing_field_static : self . backing_field_static , backing_field_owned : None , tag : self . tag . clone () , _marker : PhantomData , } } } } impl < 'computed > Label < 'computed > { # [doc = "\n        Create an owned label from this one.\n\n        This method will allocate if the label isn't based on a static string.\n        "] pub fn to_owned (& self) -> Label < 'static > { if let Some (backing_field_static) = self . backing_field_static { Label :: new (backing_field_static) } else { Label :: new_owned (self . as_str () . into ()) } } } impl Label < 'static > { # [doc = "\n        Create a new label from an owned string value.\n        "] pub fn new_owned (label : String) -> Self { let owned = Box :: into_raw (label . into_boxed_str ()) ; Label { value_computed : owned as * const str , backing_field_static : None , backing_field_owned : Some (owned) , tag : None , _marker : PhantomData , } } } }
};
}
