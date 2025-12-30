// Generated macro for try_new_scalar (function)
macro_rules! Depcrate_alloctry_new_scalar {
() => {
// Module: crate::alloc
// Provides: {"try_new_scalar"}
// Dependencies: {}
pub fn try_new_scalar < 'tcx , B : Bridge > (layout : TyAndLayout < 'tcx , Ty < 'tcx > > , scalar : Scalar , cx : & CompilerCtxt < 'tcx , B > ,) -> Result < Allocation , B :: Error > { let size = scalar . size () ; let mut allocation = Allocation :: new (size , layout . align . abi , AllocInit :: Uninit , ()) ; allocation . write_scalar (& cx . tcx , alloc_range (Size :: ZERO , size) , scalar) . map_err (| e | B :: Error :: from_internal (e)) ? ; Ok (allocation) }
};
}
