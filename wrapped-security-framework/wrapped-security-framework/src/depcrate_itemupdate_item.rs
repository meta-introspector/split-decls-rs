// Generated macro for update_item (function)
macro_rules! Depcrate_itemupdate_item {
() => {
// Module: crate::item
// Provides: {"update_item"}
// Dependencies: {}
# [doc = " Translates to `SecItemUpdate`."] pub fn update_item (search_params : & ItemSearchOptions , update_params : & ItemUpdateOptions) -> Result < () > { cvt (unsafe { SecItemUpdate (search_params . to_dictionary () . as_concrete_TypeRef () , update_params . to_dictionary () . as_concrete_TypeRef ()) }) }
};
}
