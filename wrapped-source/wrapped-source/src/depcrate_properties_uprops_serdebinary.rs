// Generated macro for binary (module)
macro_rules! Depcrate_properties_uprops_serdebinary {
() => {
// Module: crate::properties::uprops_serde
// Provides: {"binary"}
// Dependencies: {}
pub (crate) mod binary { # [derive (serde :: Deserialize)] pub (crate) struct BinaryProperty { # [serde (rename = "long_name")] pub (crate) _long_name : String , # [serde (skip)] # [serde (rename = "short_name")] pub (crate) _short_name : String , pub (crate) ranges : Vec < (u32 , u32) > , pub (crate) strings : Option < Vec < String > > , } # [derive (serde :: Deserialize)] pub (crate) struct Main { # [serde (default)] pub (crate) binary_property : Vec < BinaryProperty > , } }
};
}
