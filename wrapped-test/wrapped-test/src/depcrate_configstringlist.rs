// Generated macro for StringList (enum)
macro_rules! Depcrate_configStringList {
() => {
// Module: crate::config
// Provides: {"StringList"}
// Dependencies: {}
# [derive (Deserialize , Clone , Debug)] # [serde (untagged)] pub enum StringList { String (String) , List (Vec < String >) , }
};
}
