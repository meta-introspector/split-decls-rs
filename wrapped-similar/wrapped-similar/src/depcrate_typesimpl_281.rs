// Generated macro for impl_281 (impl)
macro_rules! Depcrate_typesimpl_281 {
() => {
// Module: crate::types
// Provides: {"impl_281"}
// Dependencies: {}
# [doc = " These methods are available for all change types."] impl < T : Clone > Change < T > { # [doc = " Returns the change tag."] pub fn tag (& self) -> ChangeTag { self . tag } # [doc = " Returns the old index if available."] pub fn old_index (& self) -> Option < usize > { self . old_index } # [doc = " Returns the new index if available."] pub fn new_index (& self) -> Option < usize > { self . new_index } # [doc = " Returns the underlying changed value."] # [doc = ""] # [doc = " Depending on the type of the underlying [`crate::text::DiffableStr`]"] # [doc = " this value is more or less useful.  If you always want to have a utf-8"] # [doc = " string it's best to use the [`Change::as_str`] and"] # [doc = " [`Change::to_string_lossy`] methods."] pub fn value (& self) -> T { self . value . clone () } # [doc = " Returns the underlying changed value as reference."] pub fn value_ref (& self) -> & T { & self . value } # [doc = " Returns the underlying changed value as mutable reference."] pub fn value_mut (& mut self) -> & mut T { & mut self . value } }
};
}
