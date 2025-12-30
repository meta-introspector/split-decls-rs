// Generated macro for PropertyValues (struct)
macro_rules! Depcrate_utilPropertyValues {
() => {
// Module: crate::util
// Provides: {"PropertyValues"}
// Dependencies: {}
# [doc = " A map from (property name, property value) to a \"canonical\" or \"long\""] # [doc = " version of the corresponding property value."] # [doc = ""] # [doc = " Property names and values are normalized according to UAX44-LM3."] # [derive (Clone , Debug)] pub struct PropertyValues { pub property : PropertyNames , pub value : BTreeMap < String , BTreeMap < String , String > > , }
};
}
