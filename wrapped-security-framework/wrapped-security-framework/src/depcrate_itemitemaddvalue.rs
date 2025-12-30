// Generated macro for ItemAddValue (enum)
macro_rules! Depcrate_itemItemAddValue {
() => {
// Module: crate::item
// Provides: {"ItemAddValue"}
// Dependencies: {}
# [doc = " Value of an item to add to the keychain."] pub enum ItemAddValue { # [doc = " Pass item by Ref (kSecValueRef)"] Ref (AddRef) , # [doc = " Pass item by Data (kSecValueData)"] Data { # [doc = " The item class (kSecClass)."] class : ItemClass , # [doc = " The item data."] data : CFData , } , }
};
}
