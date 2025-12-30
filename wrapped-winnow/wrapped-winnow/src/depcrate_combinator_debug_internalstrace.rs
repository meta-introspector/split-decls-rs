// Generated macro for Trace (struct)
macro_rules! Depcrate_combinator_debug_internalsTrace {
() => {
// Module: crate::combinator::debug::internals
// Provides: {"Trace"}
// Dependencies: {}
pub (crate) struct Trace < P , D , I , O , E > where P : Parser < I , O , E > , I : Stream , D : std :: fmt :: Display , E : ParserError < I > , { parser : P , name : D , call_count : usize , i : core :: marker :: PhantomData < I > , o : core :: marker :: PhantomData < O > , e : core :: marker :: PhantomData < E > , }
};
}
