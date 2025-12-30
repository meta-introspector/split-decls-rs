// Generated macro for PropertyValueTable (type)
macro_rules! Depcrate_propertyPropertyValueTable {
() => {
// Module: crate::property
// Provides: {"PropertyValueTable"}
// Dependencies: {}
# [doc = " Type of a property value table."] # [doc = ""] # [doc = " A property value table maps property names to a mapping of property values,"] # [doc = " where the mapping of property values is represented by a sequence of"] # [doc = " tuples. The first element of each tuple is a normalized property value"] # [doc = " while the second element of each tuple is the corresponding canonical"] # [doc = " property value."] # [doc = ""] # [doc = " Note that a property value table only includes values for properties that"] # [doc = " are catalogs, enumerations or binary properties. Properties that have"] # [doc = " string values (such as case or decomposition mappings), numeric values"] # [doc = " or are miscellaneous are not represented in this table."] pub type PropertyValueTable = & 'static [(& 'static str , PropertyValues)] ;
};
}
