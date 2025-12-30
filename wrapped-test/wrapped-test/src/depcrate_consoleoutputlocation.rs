// Generated macro for OutputLocation (enum)
macro_rules! Depcrate_consoleOutputLocation {
() => {
// Module: crate::console
// Provides: {"OutputLocation"}
// Dependencies: {}
# [doc = " Generic wrapper over stdout."] pub (crate) enum OutputLocation < T > { Pretty (Box < term :: StdoutTerminal >) , Raw (T) , }
};
}
