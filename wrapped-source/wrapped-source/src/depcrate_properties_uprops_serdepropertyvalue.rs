// Generated macro for PropertyValue (struct)
macro_rules! Depcrate_properties_uprops_serdePropertyValue {
() => {
// Module: crate::properties::uprops_serde
// Provides: {"PropertyValue"}
// Dependencies: {}
# [derive (serde :: Deserialize)] pub (crate) struct PropertyValue { pub (crate) discr : u32 , pub (crate) long : String , pub (crate) short : Option < String > , # [serde (default)] pub (crate) aliases : Vec < String > , }
};
}
