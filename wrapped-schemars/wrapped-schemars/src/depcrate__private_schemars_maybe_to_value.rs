// Generated macro for _schemars_maybe_to_value (macro)
macro_rules! Depcrate__private_schemars_maybe_to_value {
() => {
// Module: crate::_private
// Provides: {"_schemars_maybe_to_value"}
// Dependencies: {}
# [doc = " Hack to simulate specialization:"] # [doc = " `MaybeSerializeWrapper(x).maybe_to_value()` will resolve to either"] # [doc = " - The inherent method `MaybeSerializeWrapper::maybe_to_value(...)` if x is `Serialize`"] # [doc = " - The trait method `NoSerialize::maybe_to_value(...)` from the blanket impl otherwise"] # [doc (hidden)] # [macro_export] macro_rules ! _schemars_maybe_to_value { ($ expression : expr) => { { # [allow (unused_imports)] use $ crate :: _private :: { MaybeSerializeWrapper , NoSerialize as _ } ; MaybeSerializeWrapper ($ expression) . maybe_to_value () } } ; }
};
}
