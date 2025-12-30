// Generated macro for itemized_block_quote_start (function)
macro_rules! Depcrate_commentitemized_block_quote_start {
() => {
// Module: crate::comment
// Provides: {"itemized_block_quote_start"}
// Dependencies: {}
# [doc = " Determine the line_start when formatting markdown block quotes."] # [doc = " The original line_start likely contains indentation (whitespaces), which we'd like to"] # [doc = " replace with '> ' characters."] fn itemized_block_quote_start (line : & str , mut line_start : String , remove_indent : usize) -> String { let quote_level = line . chars () . take_while (| c | ! c . is_alphanumeric ()) . fold (0 , | acc , c | if c == '>' { acc + 1 } else { acc }) ; for _ in 0 .. remove_indent { line_start . pop () ; } for _ in 0 .. quote_level { line_start . push_str ("> ") } line_start }
};
}
