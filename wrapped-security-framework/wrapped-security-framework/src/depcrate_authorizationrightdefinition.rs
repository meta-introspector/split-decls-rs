// Generated macro for RightDefinition (enum)
macro_rules! Depcrate_authorizationRightDefinition {
() => {
// Module: crate::authorization
// Provides: {"RightDefinition"}
// Dependencies: {}
# [doc = " Used by `Authorization::set_item` to define the rules of he right."] # [derive (Copy , Clone)] pub enum RightDefinition < 'a > { # [doc = " The dictionary will contain the keys and values that define the rules."] FromDictionary (& 'a CFDictionary < CFStringRef , CFTypeRef >) , # [doc = " The specified right's rules will be duplicated."] FromExistingRight (& 'a str) , }
};
}
