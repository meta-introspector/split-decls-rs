// Generated macro for ParseTo (struct)
macro_rules! Depcrate_combinator_implsParseTo {
() => {
// Module: crate::combinator::impls
// Provides: {"ParseTo"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::parse_to`]"] pub struct ParseTo < P , I , O , O2 , E > where P : Parser < I , O , E > , I : Stream , O : crate :: stream :: ParseSlice < O2 > , E : ParserError < I > , { pub (crate) p : P , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) o2 : core :: marker :: PhantomData < O2 > , pub (crate) e : core :: marker :: PhantomData < E > , }
};
}
