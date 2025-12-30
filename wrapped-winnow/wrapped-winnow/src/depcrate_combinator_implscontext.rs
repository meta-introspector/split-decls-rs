// Generated macro for Context (struct)
macro_rules! Depcrate_combinator_implsContext {
() => {
// Module: crate::combinator::impls
// Provides: {"Context"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::context`]"] pub struct Context < F , I , O , E , C > where F : Parser < I , O , E > , I : Stream , E : AddContext < I , C > , E : ParserError < I > , C : Clone + core :: fmt :: Debug , { pub (crate) parser : F , pub (crate) context : C , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) e : core :: marker :: PhantomData < E > , }
};
}
