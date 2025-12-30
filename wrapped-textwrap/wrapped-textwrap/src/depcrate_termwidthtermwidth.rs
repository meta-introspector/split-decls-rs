// Generated macro for termwidth (function)
macro_rules! Depcrate_termwidthtermwidth {
() => {
// Module: crate::termwidth
// Provides: {"termwidth"}
// Dependencies: {}
# [doc = " Return the current terminal width."] # [doc = ""] # [doc = " If the terminal width cannot be determined (typically because the"] # [doc = " standard output is not connected to a terminal), a default width"] # [doc = " of 80 characters will be used."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Create an [`Options`] for wrapping at the current terminal width"] # [doc = " with a two column margin to the left and the right:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use textwrap::{termwidth, Options};"] # [doc = ""] # [doc = " let width = termwidth() - 4; // Two columns on each side."] # [doc = " let options = Options::new(width)"] # [doc = "     .initial_indent(\"  \")"] # [doc = "     .subsequent_indent(\"  \");"] # [doc = " ```"] # [doc = ""] # [doc = " **Note:** Only available when the `terminal_size` Cargo feature is"] # [doc = " enabled."] pub fn termwidth () -> usize { terminal_size :: terminal_size () . map_or (80 , | (terminal_size :: Width (w) , _) | w . into ()) }
};
}
