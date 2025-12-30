// Generated macro for define_inner_service_accessors (macro)
macro_rules! Depcrate_macrosdefine_inner_service_accessors {
() => {
// Module: crate::macros
// Provides: {"define_inner_service_accessors"}
// Dependencies: {}
# [allow (unused_macros)] macro_rules ! define_inner_service_accessors { () => { # [doc = " Gets a reference to the underlying service."] pub fn get_ref (& self) -> & S { & self . inner } # [doc = " Gets a mutable reference to the underlying service."] pub fn get_mut (& mut self) -> & mut S { & mut self . inner } # [doc = " Consumes `self`, returning the underlying service."] pub fn into_inner (self) -> S { self . inner } } ; }
};
}
