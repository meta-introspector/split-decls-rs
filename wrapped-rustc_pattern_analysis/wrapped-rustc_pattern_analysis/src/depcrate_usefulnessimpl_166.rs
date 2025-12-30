// Generated macro for impl_166 (impl)
macro_rules! Depcrate_usefulnessimpl_166 {
() => {
// Module: crate::usefulness
// Provides: {"impl_166"}
// Dependencies: {}
impl < 'a , Cx : PatCx > PlaceCtxt < 'a , Cx > { fn ctor_arity (& self , ctor : & Constructor < Cx >) -> usize { self . cx . ctor_arity (ctor , self . ty) } fn wild_from_ctor (& self , ctor : Constructor < Cx >) -> WitnessPat < Cx > { WitnessPat :: wild_from_ctor (self . cx , ctor , self . ty . clone ()) } }
};
}
