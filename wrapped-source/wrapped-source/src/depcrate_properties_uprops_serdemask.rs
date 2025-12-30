// Generated macro for mask (module)
macro_rules! Depcrate_properties_uprops_serdemask {
() => {
// Module: crate::properties::uprops_serde
// Provides: {"mask"}
// Dependencies: {}
pub (crate) mod mask { # [derive (serde :: Deserialize)] pub (crate) struct MaskPropertyMap { # [serde (rename = "long_name")] pub (crate) _long_name : String , # [serde (rename = "short_name")] pub (crate) _short_name : String , # [serde (rename = "mask_for")] pub (crate) _mask_for : String , pub (crate) values : Vec < super :: PropertyValue > , } # [derive (serde :: Deserialize)] pub (crate) struct Main { # [serde (default)] pub (crate) mask_property : Vec < MaskPropertyMap > , } }
};
}
