// Generated macro for PropertyTable (type)
macro_rules! Depcrate_propertyPropertyTable {
() => {
// Module: crate::property
// Provides: {"PropertyTable"}
// Dependencies: {}
# [doc = " The type of a property name table."] # [doc = ""] # [doc = " A property name table is a sequence of sorted tuples, where the first"] # [doc = " value in each tuple is a normalized property name and the second value of"] # [doc = " each tuple is the corresponding canonical property name."] pub type PropertyTable = & 'static [(& 'static str , & 'static str)] ;
};
}
