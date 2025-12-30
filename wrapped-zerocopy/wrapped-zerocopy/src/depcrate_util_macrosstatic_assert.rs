// Generated macro for static_assert (macro)
macro_rules! Depcrate_util_macrosstatic_assert {
() => {
// Module: crate::util::macros
// Provides: {"static_assert"}
// Dependencies: {}
# [doc = " Asserts at compile time that `$condition` is true for `Self` or the given"] # [doc = " `$tyvar`s. Unlike `const_assert`, this is *strictly* a compile-time check;"] # [doc = " it cannot be evaluated in a runtime context. The condition is checked after"] # [doc = " monomorphization and, upon failure, emits a compile error."] macro_rules ! static_assert { (Self $ (: $ (? $ optbound : ident $ (+) ?) * $ ($ bound : ident $ (+) ?) *) ? => $ condition : expr $ (, $ args : tt) *) => { { trait StaticAssert { const ASSERT : bool ; } impl < T $ (: $ (? $ optbound +) * $ ($ bound +) *) ?> StaticAssert for T { const ASSERT : bool = { const_assert ! ($ condition $ (, $ args) *) ; $ condition } ; } const_assert ! (< Self as StaticAssert >:: ASSERT) ; } } ; ($ ($ tyvar : ident $ (: $ (? $ optbound : ident $ (+) ?) * $ ($ bound : ident $ (+) ?) *) ?) ,* => $ condition : expr $ (, $ args : tt) *) => { { trait StaticAssert { const ASSERT : bool ; } impl <$ ($ tyvar $ (: $ (? $ optbound +) * $ ($ bound +) *) ?,) *> StaticAssert for ($ (core :: marker :: PhantomData <$ tyvar >,) *) { const ASSERT : bool = { const_assert ! ($ condition $ (, $ args) *) ; $ condition } ; } const_assert ! (< ($ (core :: marker :: PhantomData <$ tyvar >,) *) as StaticAssert >:: ASSERT) ; } } ; }
};
}
