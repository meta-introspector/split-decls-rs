// Generated macro for ItemUpdateValue (enum)
macro_rules! Depcrate_itemItemUpdateValue {
() => {
// Module: crate::item
// Provides: {"ItemUpdateValue"}
// Dependencies: {}
# [doc = " Value of an item to update in the keychain."] pub enum ItemUpdateValue { # [doc = " Pass item by Ref (kSecValueRef)"] Ref (AddRef) , # [doc = " Pass item by Data (kSecValueData)"] # [doc = ""] # [doc = " Note that if the [`ItemClass`] of the updated data is different to the original data"] # [doc = " stored in the keychain, it should be specified using [`ItemUpdateOptions::set_class`]."] Data (CFData) , }
};
}
