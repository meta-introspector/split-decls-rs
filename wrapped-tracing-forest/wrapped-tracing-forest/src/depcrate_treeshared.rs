// Generated macro for Shared (struct)
macro_rules! Depcrate_treeShared {
() => {
// Module: crate::tree
// Provides: {"Shared"}
// Dependencies: {}
# [derive (Clone , Debug)] # [cfg_attr (feature = "serde" , derive (Serialize))] pub (crate) struct Shared { # [doc = " The ID of the event or span."] # [cfg (feature = "uuid")] pub (crate) uuid : Uuid , # [doc = " When the event occurred or when the span opened."] # [cfg (feature = "chrono")] # [cfg_attr (feature = "serde" , serde (serialize_with = "ser::timestamp"))] pub (crate) timestamp : DateTime < Utc > , # [doc = " The level the event or span occurred at."] # [cfg_attr (feature = "serde" , serde (serialize_with = "ser::level"))] pub (crate) level : Level , # [doc = " Key-value data."] # [cfg_attr (feature = "serde" , serde (serialize_with = "ser::fields"))] pub (crate) fields : FieldSet , }
};
}
