// Generated macro for impl_247 (impl)
macro_rules! Depcrate_shimimpl_247 {
() => {
// Module: crate::shim
// Provides: {"impl_247"}
// Dependencies: {}
impl < 'tcx > MutVisitor < 'tcx > for FixProxyFutureDropVisitor < 'tcx > { fn tcx (& self) -> TyCtxt < 'tcx > { self . tcx } fn visit_place (& mut self , place : & mut Place < 'tcx > , _context : PlaceContext , _location : Location ,) { if place . local == Local :: from_u32 (1) { if place . projection . len () == 1 { assert ! (matches ! (place . projection . first () , Some (ProjectionElem :: Field (FieldIdx :: ZERO , _)))) ; * place = Place :: from (self . replace_to) ; } else if place . projection . len () == 2 { assert ! (matches ! (place . projection [0] , ProjectionElem :: Field (FieldIdx :: ZERO , _))) ; assert ! (matches ! (place . projection [1] , ProjectionElem :: Deref)) ; * place = Place :: from (self . replace_to) . project_deeper (& [ProjectionElem :: Deref] , self . tcx) ; } } } }
};
}
