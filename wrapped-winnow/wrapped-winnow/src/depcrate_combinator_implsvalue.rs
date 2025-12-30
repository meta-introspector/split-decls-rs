// Generated macro for Value (struct)
macro_rules! Depcrate_combinator_implsValue {
() => {
// Module: crate::combinator::impls
// Provides: {"Value"}
// Dependencies: {}
# [doc = " [`Parser`] implementation for [`Parser::value`]"] pub struct Value < F , I , O , O2 , E > where F : Parser < I , O , E > , O2 : Clone , { pub (crate) parser : F , pub (crate) val : O2 , pub (crate) i : core :: marker :: PhantomData < I > , pub (crate) o : core :: marker :: PhantomData < O > , pub (crate) e : core :: marker :: PhantomData < E > , }
};
}
