// Generated macro for TestName (enum)
macro_rules! Depcrate_typesTestName {
() => {
// Module: crate::types
// Provides: {"TestName"}
// Dependencies: {}
# [derive (Clone , PartialEq , Eq , Hash , Debug)] pub enum TestName { StaticTestName (& 'static str) , DynTestName (String) , AlignedTestName (Cow < 'static , str > , NamePadding) , }
};
}
