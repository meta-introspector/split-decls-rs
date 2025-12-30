// Generated macro for always_update (function)
macro_rules! Depcrate_updatealways_update {
() => {
// Module: crate::update
// Provides: {"always_update"}
// Dependencies: {}
# [doc = " Helper for generated code. Updates `*old_pointer` with `new_value`"] # [doc = " and updates `*old_revision` with `new_revision.` Used for fields"] # [doc = " tagged with `#[no_eq]`"] pub fn always_update < T > (old_revision : & mut Revision , new_revision : Revision , old_pointer : & mut T , new_value : T ,) { * old_revision = new_revision ; * old_pointer = new_value ; }
};
}
