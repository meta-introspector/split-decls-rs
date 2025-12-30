// Generated macro for assert_type (macro)
macro_rules! Depcrateassert_type {
() => {
// Module: crate
// Provides: {"assert_type"}
// Dependencies: {}
# [doc = " Asserts that a type is `True`, aka `B1`."] # [macro_export] macro_rules ! assert_type { ($ a : ty) => { const _ : core :: marker :: PhantomData <<$ a as $ crate :: Same < True >>:: Output > = core :: marker :: PhantomData ; } ; }
};
}
