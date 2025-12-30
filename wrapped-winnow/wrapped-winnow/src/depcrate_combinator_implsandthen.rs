// Generated macro for AndThen (struct)
macro_rules! Depcrate_combinator_implsAndThen {
() => {
// Module: crate::combinator::impls
// Provides: {"AndThen"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::and_then`]"] pub struct AndThen < F , G , I , O , O2 , E > where F : Parser < I , O , E > , G : Parser < O , O2 , E > , O : StreamIsPartial , I : Stream , { pub (crate) outer : F , pub (crate) inner : G , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) o2 : core :: marker :: PhantomData < O2 > , pub (crate) e : core :: marker :: PhantomData < E > , }
};
}
