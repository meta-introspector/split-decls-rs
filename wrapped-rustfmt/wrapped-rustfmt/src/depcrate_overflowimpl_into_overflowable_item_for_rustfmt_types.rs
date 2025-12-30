// Generated macro for impl_into_overflowable_item_for_rustfmt_types (macro)
macro_rules! Depcrate_overflowimpl_into_overflowable_item_for_rustfmt_types {
() => {
// Module: crate::overflow
// Provides: {"impl_into_overflowable_item_for_rustfmt_types"}
// Dependencies: {}
macro_rules ! impl_into_overflowable_item_for_rustfmt_types { ([$ ($ ty : ident) ,*] , [$ ($ ty_with_lifetime : ident) ,*]) => { $ (impl <'a > IntoOverflowableItem <'a > for $ ty { fn into_overflowable_item (&'a self) -> OverflowableItem <'a > { OverflowableItem ::$ ty (self) } }) * $ (impl <'a > IntoOverflowableItem <'a > for $ ty_with_lifetime <'a > { fn into_overflowable_item (&'a self) -> OverflowableItem <'a > { OverflowableItem ::$ ty_with_lifetime (self) } }) * } }
};
}
