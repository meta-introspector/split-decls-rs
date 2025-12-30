// Generated macro for ItemizedBlock (struct)
macro_rules! Depcrate_commentItemizedBlock {
() => {
// Module: crate::comment
// Provides: {"ItemizedBlock"}
// Dependencies: {}
# [doc = " Block that is formatted as an item."] # [doc = ""] # [doc = " An item starts with either a star `*`, a dash `-`, a greater-than `>`, a plus '+', or a number"] # [doc = " `12.` or `34)` (with at most 2 digits). An item represents CommonMark's [\"list"] # [doc = " items\"](https://spec.commonmark.org/0.30/#list-items) and/or [\"block"] # [doc = " quotes\"](https://spec.commonmark.org/0.30/#block-quotes), but note that only a subset of"] # [doc = " CommonMark is recognized - see the doc comment of [`ItemizedBlock::get_marker_length`] for more"] # [doc = " details."] # [doc = ""] # [doc = " Different level of indentation are handled by shrinking the shape accordingly."] struct ItemizedBlock { # [doc = " the lines that are identified as part of an itemized block"] lines : Vec < String > , # [doc = " the number of characters (typically whitespaces) up to the item marker"] indent : usize , # [doc = " the string that marks the start of an item"] opener : String , # [doc = " sequence of characters (typically whitespaces) to prefix new lines that are part of the item"] line_start : String , }
};
}
