// Generated macro for impl_79 (impl)
macro_rules! Depcrate_termwidthimpl_79 {
() => {
// Module: crate::termwidth
// Provides: {"impl_79"}
// Dependencies: {}
impl < 'a > Options < 'a > { # [doc = " Creates a new [`Options`] with `width` set to the current"] # [doc = " terminal width. If the terminal width cannot be determined"] # [doc = " (typically because the standard input and output is not"] # [doc = " connected to a terminal), a width of 80 characters will be"] # [doc = " used. Other settings use the same defaults as"] # [doc = " [`Options::new`]."] # [doc = ""] # [doc = " Equivalent to:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use textwrap::{termwidth, Options};"] # [doc = ""] # [doc = " let options = Options::new(termwidth());"] # [doc = " ```"] # [doc = ""] # [doc = " **Note:** Only available when the `terminal_size` feature is"] # [doc = " enabled."] pub fn with_termwidth () -> Self { Self :: new (termwidth ()) } }
};
}
