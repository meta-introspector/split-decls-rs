// Generated macro for update_fallback (function)
macro_rules! Depcrate_updateupdate_fallback {
() => {
// Module: crate::update
// Provides: {"update_fallback"}
// Dependencies: {}
# [doc = " \"Fallback\" for maybe-update that is suitable for fully owned T"] # [doc = " that implement `Eq`. In this version, we update only if the new value"] # [doc = " is not `Eq` to the old one. Note that given `Eq` impls that are not just"] # [doc = " structurally comparing fields, this may cause us not to update even if"] # [doc = " the value has changed (presumably because this change is not semantically"] # [doc = " significant)."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " See `Update::maybe_update`"] pub unsafe fn update_fallback < T > (old_pointer : * mut T , new_value : T) -> bool where T : 'static + PartialEq , { let old_ref : & mut T = unsafe { & mut * old_pointer } ; if * old_ref != new_value { * old_ref = new_value ; true } else { false } }
};
}
