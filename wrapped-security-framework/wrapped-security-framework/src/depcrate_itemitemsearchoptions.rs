// Generated macro for ItemSearchOptions (struct)
macro_rules! Depcrate_itemItemSearchOptions {
() => {
// Module: crate::item
// Provides: {"ItemSearchOptions"}
// Dependencies: {}
# [doc = " A builder type to search for items in keychains."] # [derive (Default)] pub struct ItemSearchOptions { # [cfg (target_os = "macos")] keychains : Option < CFArray < SecKeychain > > , # [cfg (not (target_os = "macos"))] keychains : Option < CFArray < CFType > > , ignore_legacy_keychains : bool , case_insensitive : Option < bool > , class : Option < ItemClass > , key_class : Option < KeyClass > , load_refs : bool , load_attributes : bool , load_data : bool , limit : Option < Limit > , trusted_only : Option < bool > , label : Option < CFString > , service : Option < CFString > , subject : Option < CFString > , account : Option < CFString > , access_group : Option < CFString > , cloud_sync : Option < CloudSync > , pub_key_hash : Option < CFData > , serial_number : Option < CFData > , app_label : Option < CFData > , # [cfg (any (feature = "OSX_10_13" , target_os = "ios" , target_os = "tvos" , target_os = "watchos" , target_os = "visionos"))] authentication_context : Option < CFType > , # [cfg (any (feature = "OSX_10_12" , target_os = "ios" , target_os = "tvos" , target_os = "watchos" , target_os = "visionos"))] skip_authenticated_items : bool , }
};
}
