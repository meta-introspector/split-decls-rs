// Generated macro for add_item (function)
macro_rules! Depcrate_itemadd_item {
() => {
// Module: crate::item
// Provides: {"add_item"}
// Dependencies: {}
# [doc = " Translates to `SecItemAdd`. Use `ItemAddOptions` to build an `add_params`"] # [doc = " `CFDictionary`."] # [deprecated (since = "3.0.0" , note = "use `ItemAddOptions::add` instead")] # [allow (deprecated)] pub fn add_item (add_params : CFDictionary) -> Result < () > { cvt (unsafe { SecItemAdd (add_params . as_concrete_TypeRef () , std :: ptr :: null_mut ()) }) }
};
}
