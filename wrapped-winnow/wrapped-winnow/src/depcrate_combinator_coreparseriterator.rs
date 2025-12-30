// Generated macro for ParserIterator (struct)
macro_rules! Depcrate_combinator_coreParserIterator {
() => {
// Module: crate::combinator::core
// Provides: {"ParserIterator"}
// Dependencies: {}
# [doc = " Main structure associated to [`iterator`]."] pub struct ParserIterator < F , I , O , E > where F : Parser < I , O , E > , I : Stream , { parser : F , input : I , state : State < E > , o : core :: marker :: PhantomData < O > , }
};
}
