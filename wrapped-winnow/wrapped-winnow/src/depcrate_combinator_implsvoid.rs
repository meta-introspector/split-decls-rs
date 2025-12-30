// Generated macro for Void (struct)
macro_rules! Depcrate_combinator_implsVoid {
() => {
// Module: crate::combinator::impls
// Provides: {"Void"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::void`]"] pub struct Void < F , I , O , E > where F : Parser < I , O , E > , { pub (crate) parser : F , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) e : core :: marker :: PhantomData < E > , }
};
}
