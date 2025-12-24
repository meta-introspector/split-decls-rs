use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Specification for the formatting of an argument in the format string.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct FormatSpec<'input> {
    /// Optionally specified character to fill alignment with.
    pub fill: Option<char>,
    /// Span of the optionally specified fill character.
    pub fill_span: Option<Range<usize>>,
    /// Optionally specified alignment.
    pub align: Alignment,
    /// The `+` or `-` flag.
    pub sign: Option<Sign>,
    /// The `#` flag.
    pub alternate: bool,
    /// The `0` flag.
    pub zero_pad: bool,
    /// The `x` or `X` flag. (Only for `Debug`.)
    pub debug_hex: Option<DebugHex>,
    /// The integer precision to use.
    pub precision: Count<'input>,
    /// The span of the precision formatting flag (for diagnostics).
    pub precision_span: Option<Range<usize>>,
    /// The string width requested for the resulting format.
    pub width: Count<'input>,
    /// The span of the width formatting flag (for diagnostics).
    pub width_span: Option<Range<usize>>,
    /// The descriptor string representing the name of the format desired for
    /// this argument, this can be empty or any number of characters, although
    /// it is required to be one word.
    pub ty: &'input str,
    /// The span of the descriptor string (for diagnostics).
    pub ty_span: Option<Range<usize>>,
}
