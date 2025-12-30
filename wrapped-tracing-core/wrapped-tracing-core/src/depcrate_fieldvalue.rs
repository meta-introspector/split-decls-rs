// Generated macro for Value (trait)
macro_rules! Depcrate_fieldValue {
() => {
// Module: crate::field
// Provides: {"Value"}
// Dependencies: {}
# [doc = " A field value of an erased type."] # [doc = ""] # [doc = " Implementors of `Value` may call the appropriate typed recording methods on"] # [doc = " the [visitor] passed to their `record` method in order to indicate how"] # [doc = " their data should be recorded."] # [doc = ""] # [doc = " [visitor]: Visit"] pub trait Value : crate :: sealed :: Sealed { # [doc = " Visits this value with the given `Visitor`."] fn record (& self , key : & Field , visitor : & mut dyn Visit) ; }
};
}
