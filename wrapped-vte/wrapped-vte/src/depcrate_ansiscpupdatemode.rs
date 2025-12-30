// Generated macro for ScpUpdateMode (enum)
macro_rules! Depcrate_ansiScpUpdateMode {
() => {
// Module: crate::ansi
// Provides: {"ScpUpdateMode"}
// Dependencies: {}
# [doc = " SCP control's second parameter which determines update mode/direction"] # [doc = " between components."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub enum ScpUpdateMode { # [doc = " SCP's second parameter value of 0 (the default). Implementation"] # [doc = " dependant update."] ImplementationDependant , # [doc = " SCP's second parameter value of 1."] # [doc = ""] # [doc = " Reflect data component changes in the presentation component."] DataToPresentation , # [doc = " SCP's second parameter value of 2."] # [doc = ""] # [doc = " Reflect presentation component changes in the data component."] PresentationToData , }
};
}
