// Generated macro for macro_35 (macro)
macro_rules! Depcrate_builtinmacro_35 {
() => {
// Module: crate::builtin
// Provides: {"macro_35"}
// Dependencies: {}
declare_lint ! { # [doc = " The `dead_code` lint detects unused, unexported items."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " fn foo() {}"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " Dead code may signal a mistake or unfinished code. To silence the"] # [doc = " warning for individual items, prefix the name with an underscore such"] # [doc = " as `_foo`. If it was intended to expose the item outside of the crate,"] # [doc = " consider adding a visibility modifier like `pub`."] # [doc = ""] # [doc = " To preserve the numbering of tuple structs with unused fields,"] # [doc = " change the unused fields to have unit type or use"] # [doc = " `PhantomData`."] # [doc = ""] # [doc = " Otherwise consider removing the unused code."] # [doc = ""] # [doc = " ### Limitations"] # [doc = ""] # [doc = " Removing fields that are only used for side-effects and never"] # [doc = " read will result in behavioral changes. Examples of this"] # [doc = " include:"] # [doc = ""] # [doc = " - If a field's value performs an action when it is dropped."] # [doc = " - If a field's type does not implement an auto trait"] # [doc = "   (e.g. `Send`, `Sync`, `Unpin`)."] # [doc = ""] # [doc = " For side-effects from dropping field values, this lint should"] # [doc = " be allowed on those fields. For side-effects from containing"] # [doc = " field types, `PhantomData` should be used."] pub DEAD_CODE , Warn , "detect unused, unexported items" }
};
}
