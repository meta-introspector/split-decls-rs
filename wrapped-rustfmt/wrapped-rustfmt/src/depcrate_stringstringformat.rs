// Generated macro for StringFormat (struct)
macro_rules! Depcrate_stringStringFormat {
() => {
// Module: crate::string
// Provides: {"StringFormat"}
// Dependencies: {}
# [doc = " Describes the layout of a piece of text."] pub (crate) struct StringFormat < 'a > { # [doc = " The opening sequence of characters for the piece of text"] pub (crate) opener : & 'a str , # [doc = " The closing sequence of characters for the piece of text"] pub (crate) closer : & 'a str , # [doc = " The opening sequence of characters for a line"] pub (crate) line_start : & 'a str , # [doc = " The closing sequence of characters for a line"] pub (crate) line_end : & 'a str , # [doc = " The allocated box to fit the text into"] pub (crate) shape : Shape , # [doc = " Trim trailing whitespaces"] pub (crate) trim_end : bool , pub (crate) config : & 'a Config , }
};
}
