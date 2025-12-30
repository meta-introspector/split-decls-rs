// Generated macro for RawConverter (struct)
macro_rules! DepcrateRawConverter {
() => {
// Module: crate
// Provides: {"RawConverter"}
// Dependencies: {}
# [doc = " A raw token (straight from lexer) converter"] struct RawConverter < 'a , Ctx > { lexed : parser :: LexedStr < 'a > , pos : usize , anchor : SpanAnchor , ctx : Ctx , mode : DocCommentDesugarMode , }
};
}
