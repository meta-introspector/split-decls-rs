// Generated macro for impl_67 (impl)
macro_rules! Depcrate_exprimpl_67 {
() => {
// Module: crate::expr
// Provides: {"impl_67"}
// Dependencies: {}
impl Expr { pub fn eval (& self , rustc : Version) -> bool { use self :: Expr :: * ; match self { Stable => rustc . channel == Channel :: Stable , Beta => rustc . channel == Channel :: Beta , Nightly => match rustc . channel { Channel :: Nightly (_) | Channel :: Dev => true , Channel :: Stable | Channel :: Beta => false , } , Date (date) => match rustc . channel { Channel :: Nightly (rustc) => rustc == * date , Channel :: Stable | Channel :: Beta | Channel :: Dev => false , } , Since (bound) => rustc >= * bound , Before (bound) => rustc < * bound , Release (release) => { rustc . channel == Channel :: Stable && rustc . minor == release . minor && release . patch . map_or (true , | patch | rustc . patch == patch) } Not (expr) => ! expr . eval (rustc) , Any (exprs) => exprs . iter () . any (| e | e . eval (rustc)) , All (exprs) => exprs . iter () . all (| e | e . eval (rustc)) , } } }
};
}
