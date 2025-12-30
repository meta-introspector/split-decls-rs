// Generated macro for StaticRawConverter (struct)
macro_rules! DepcrateStaticRawConverter {
() => {
// Module: crate
// Provides: {"StaticRawConverter"}
// Dependencies: {}
# [doc = " A raw token (straight from lexer) converter that gives every token the same span."] struct StaticRawConverter < 'a , S > { lexed : parser :: LexedStr < 'a > , pos : usize , span : S , mode : DocCommentDesugarMode , }
};
}
