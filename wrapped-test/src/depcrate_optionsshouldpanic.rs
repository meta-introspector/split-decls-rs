// Generated macro for ShouldPanic (enum)
macro_rules! Depcrate_optionsShouldPanic {
() => {
// Module: crate::options
// Provides: {"ShouldPanic"}
// Dependencies: {}
# [doc = " Whether test is expected to panic or not"] # [derive (Copy , Clone , Debug , PartialEq , Eq , Hash)] pub enum ShouldPanic { No , Yes , YesWithMessage (& 'static str) , }
};
}
