macro_rules! deps {
    () => {
        Piece!();
        ParseMode!();
        ParseError!();
    };
}

macro_rules! Parser {
    () => {
        deps!();
        # [doc = " The parser structure for interpreting the input format string. This is"] # [doc = " modeled as an iterator over `Piece` structures to form a stream of tokens"] # [doc = " being output."] # [doc = ""] # [doc = " This is a recursive-descent parser for the sake of simplicity, and if"] # [doc = " necessary there's probably lots of room for improvement performance-wise."] pub struct Parser < 'input > { mode : ParseMode , # [doc = " Input to be parsed"] input : & 'input str , # [doc = " Tuples of the span in the code snippet (input as written before being unescaped), the pos in input, and the char in input"] input_vec : Vec < (Range < usize > , usize , char) > , # [doc = " Index into input_vec"] input_vec_index : usize , # [doc = " Error messages accumulated during parsing"] pub errors : Vec < ParseError > , # [doc = " Current position of implicit positional argument pointer"] pub curarg : usize , # [doc = " Start and end byte offset of every successfully parsed argument"] pub arg_places : Vec < Range < usize > > , # [doc = " Span of the last opening brace seen, used for error reporting"] last_open_brace : Option < Range < usize > > , # [doc = " Whether this formatting string was written directly in the source. This controls whether we"] # [doc = " can use spans to refer into it and give better error messages."] # [doc = " N.B: This does _not_ control whether implicit argument captures can be used."] pub is_source_literal : bool , # [doc = " Index to the end of the literal snippet"] end_of_snippet : usize , # [doc = " Start position of the current line."] cur_line_start : usize , # [doc = " Start and end byte offset of every line of the format string. Excludes"] # [doc = " newline characters and leading whitespace."] pub line_spans : Vec < Range < usize > > , }
    };
}

Parser!()