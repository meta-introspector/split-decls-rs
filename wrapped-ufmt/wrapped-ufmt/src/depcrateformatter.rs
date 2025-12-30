// Generated macro for Formatter (struct)
macro_rules! DepcrateFormatter {
() => {
// Module: crate
// Provides: {"Formatter"}
// Dependencies: {}
# [doc = " Configuration for formatting"] # [allow (non_camel_case_types)] pub struct Formatter < 'w , W > where W : uWrite + ? Sized , { indentation : u8 , pretty : bool , writer : & 'w mut W , }
};
}
