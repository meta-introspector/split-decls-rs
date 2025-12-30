// Generated macro for ForLoopValues (enum)
macro_rules! Depcrate_renderer_for_loopForLoopValues {
() => {
// Module: crate::renderer::for_loop
// Provides: {"ForLoopValues"}
// Dependencies: {}
# [doc = " Enumerates on the types of values to be iterated, scalars and pairs"] # [derive (Debug)] pub enum ForLoopValues < 'a > { # [doc = " Values for an array style iteration"] Array (Val < 'a >) , # [doc = " Values for a per-character iteration on a string"] String (Val < 'a >) , # [doc = " Values for an object style iteration"] Object (Vec < (String , Val < 'a >) >) , }
};
}
