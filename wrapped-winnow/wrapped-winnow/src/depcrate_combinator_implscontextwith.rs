// Generated macro for ContextWith (struct)
macro_rules! Depcrate_combinator_implsContextWith {
() => {
// Module: crate::combinator::impls
// Provides: {"ContextWith"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::context`]"] pub struct ContextWith < P , I , O , E , F , C , FI > where P : Parser < I , O , E > , I : Stream , E : AddContext < I , C > , E : ParserError < I > , F : Fn () -> FI + Clone , C : core :: fmt :: Debug , FI : Iterator < Item = C > , { pub (crate) parser : P , pub (crate) context : F , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) e : core :: marker :: PhantomData < E > , pub (crate) c : core :: marker :: PhantomData < C > , pub (crate) fi : core :: marker :: PhantomData < FI > , }
};
}
