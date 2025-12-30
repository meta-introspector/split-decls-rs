// Generated macro for ItemUpdateOptions (struct)
macro_rules! Depcrate_itemItemUpdateOptions {
() => {
// Module: crate::item
// Provides: {"ItemUpdateOptions"}
// Dependencies: {}
# [doc = " Builder-pattern struct for specifying options for `update_item` (`SecUpdateItem`"] # [doc = " wrapper)."] # [doc = ""] # [doc = " When finished populating options call [`update_item`]."] # [derive (Default)] pub struct ItemUpdateOptions { # [doc = " Optional value (by ref or data) of the item to update."] pub value : Option < ItemUpdateValue > , # [doc = " Optional kSecAttrAccount attribute."] pub account_name : Option < CFString > , # [doc = " Optional kSecAttrAccessGroup attribute."] pub access_group : Option < CFString > , # [doc = " Optional kSecAttrComment attribute."] pub comment : Option < CFString > , # [doc = " Optional kSecAttrDescription attribute."] pub description : Option < CFString > , # [doc = " Optional kSecAttrLabel attribute."] pub label : Option < CFString > , # [doc = " Optional kSecAttrService attribute."] pub service : Option < CFString > , # [doc = " Optional keychain location."] pub location : Option < Location > , # [doc = " Optional kSecClass."] # [doc = ""] # [doc = " Overwrites `value`'s class if set."] pub class : Option < ItemClass > , }
};
}
