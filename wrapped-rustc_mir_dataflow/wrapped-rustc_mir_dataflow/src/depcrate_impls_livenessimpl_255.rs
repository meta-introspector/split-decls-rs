// Generated macro for impl_255 (impl)
macro_rules! Depcrate_impls_livenessimpl_255 {
() => {
// Module: crate::impls::liveness
// Provides: {"impl_255"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for TransferFunction < '_ > { fn visit_place (& mut self , place : & mir :: Place < 'tcx > , context : PlaceContext , location : Location) { if let PlaceContext :: MutatingUse (MutatingUseContext :: Yield) = context { return ; } match DefUse :: for_place (* place , context) { DefUse :: Def => { if let PlaceContext :: MutatingUse (MutatingUseContext :: Call | MutatingUseContext :: AsmOutput ,) = context { } else { self . 0 . kill (place . local) ; } } DefUse :: Use => self . 0 . gen_ (place . local) , DefUse :: PartialWrite | DefUse :: NonUse => { } } self . visit_projection (place . as_ref () , context , location) ; } fn visit_local (& mut self , local : Local , context : PlaceContext , _ : Location) { DefUse :: apply (self . 0 , local . into () , context) ; } }
};
}
