// Generated macro for ItemAddOptions (struct)
macro_rules! Depcrate_itemItemAddOptions {
() => {
// Module: crate::item
// Provides: {"ItemAddOptions"}
// Dependencies: {}
# [doc = " Builder-pattern struct for specifying options for `add_item` (`SecAddItem`"] # [doc = " wrapper)."] # [doc = ""] # [doc = " When finished populating options call [`Self::add`]."] pub struct ItemAddOptions { # [doc = " The value (by ref or data) of the item to add, required."] pub value : ItemAddValue , # [doc = " Optional kSecAttrAccount attribute."] pub account_name : Option < CFString > , # [doc = " Optional kSecAttrAccessGroup attribute."] pub access_group : Option < CFString > , # [doc = " Optional kSecAttrComment attribute."] pub comment : Option < CFString > , # [doc = " Optional kSecAttrDescription attribute."] pub description : Option < CFString > , # [doc = " Optional kSecAttrLabel attribute."] pub label : Option < CFString > , # [doc = " Optional kSecAttrService attribute."] pub service : Option < CFString > , # [doc = " Optional keychain location."] pub location : Option < Location > , }
};
}
