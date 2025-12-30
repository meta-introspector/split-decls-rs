// Generated macro for assert_type_eq (macro)
macro_rules! Depcrateassert_type_eq {
() => {
// Module: crate
// Provides: {"assert_type_eq"}
// Dependencies: {}
# [doc = " Asserts that two types are the same."] # [macro_export] macro_rules ! assert_type_eq { ($ a : ty , $ b : ty) => { const _ : core :: marker :: PhantomData <<$ a as $ crate :: Same <$ b >>:: Output > = core :: marker :: PhantomData ; } ; }
};
}
