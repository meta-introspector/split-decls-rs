// Generated macro for Indent (struct)
macro_rules! Depcrate_scannerIndent {
() => {
// Module: crate::scanner
// Provides: {"Indent"}
// Dependencies: {}
# [doc = " An indentation level on the stack of indentations."] # [derive (Clone , Debug , Default)] struct Indent { # [doc = " The former indentation level."] indent : isize , # [doc = " Whether, upon closing, this indents generates a `BlockEnd` token."] # [doc = ""] # [doc = " There are levels of indentation which do not start a block. Examples of this would be:"] # [doc = " ```yaml"] # [doc = " -"] # [doc = "   foo # ok"] # [doc = " -"] # [doc = " bar # ko, bar needs to be indented further than the `-`."] # [doc = " - ["] # [doc = "  baz, # ok"] # [doc = " quux # ko, quux needs to be indented further than the '-'."] # [doc = " ] # ko, the closing bracket needs to be indented further than the `-`."] # [doc = " ```"] # [doc = ""] # [doc = " The indentation level created by the `-` is for a single entry in the sequence. Emitting a"] # [doc = " `BlockEnd` when this indentation block ends would generate one `BlockEnd` per entry in the"] # [doc = " sequence, although we must have exactly one to end the sequence."] needs_block_end : bool , }
};
}
