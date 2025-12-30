// Generated macro for ErrInto (struct)
macro_rules! Depcrate_combinator_implsErrInto {
() => {
// Module: crate::combinator::impls
// Provides: {"ErrInto"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::err_into`]"] pub struct ErrInto < F , I , O , E , E2 > where F : Parser < I , O , E > , E : Into < E2 > , { pub (crate) parser : F , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) e : core :: marker :: PhantomData < E > , pub (crate) e2 : core :: marker :: PhantomData < E2 > , }
};
}
