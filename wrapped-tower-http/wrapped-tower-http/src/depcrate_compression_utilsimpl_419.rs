// Generated macro for impl_419 (impl)
macro_rules! Depcrate_compression_utilsimpl_419 {
() => {
// Module: crate::compression_utils
// Provides: {"impl_419"}
// Dependencies: {}
# [allow (dead_code)] impl < B > BodyIntoStream < B > where B : Body , { pub (crate) fn new (body : B) -> Self { Self { body , yielded_all_data : false , non_data_frame : None , } } # [doc = " Get a reference to the inner body"] pub (crate) fn get_ref (& self) -> & B { & self . body } # [doc = " Get a mutable reference to the inner body"] pub (crate) fn get_mut (& mut self) -> & mut B { & mut self . body } # [doc = " Get a pinned mutable reference to the inner body"] pub (crate) fn get_pin_mut (self : Pin < & mut Self >) -> Pin < & mut B > { self . project () . body } # [doc = " Consume `self`, returning the inner body"] pub (crate) fn into_inner (self) -> B { self . body } }
};
}
