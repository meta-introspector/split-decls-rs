// Generated macro for SkipTabs (enum)
macro_rules! Depcrate_scannerSkipTabs {
() => {
// Module: crate::scanner
// Provides: {"SkipTabs"}
// Dependencies: {}
# [doc = " Behavior to adopt regarding treating tabs as whitespace."] # [doc = ""] # [doc = " Although tab is a valid yaml whitespace, it doesn't always behave the same as a space."] # [derive (Copy , Clone , Eq , PartialEq)] enum SkipTabs { # [doc = " Skip all tabs as whitespace."] Yes , # [doc = " Don't skip any tab. Return from the function when encountering one."] No , # [doc = " Return value from the function."] Result (# [doc = " Whether tabs were encountered."] bool , # [doc = " Whether at least 1 valid yaml whitespace has been encountered."] bool ,) , }
};
}
