// Generated macro for impl_241 (impl)
macro_rules! Depcrate_impls_initializedimpl_241 {
() => {
// Module: crate::impls::initialized
// Provides: {"impl_241"}
// Dependencies: {}
impl < 'tcx > MaybeUninitializedPlaces < '_ , 'tcx > { fn update_bits (state : & mut < Self as Analysis < 'tcx > > :: Domain , path : MovePathIndex , dfstate : DropFlagState ,) { match dfstate { DropFlagState :: Absent => state . gen_ (path) , DropFlagState :: Present => state . kill (path) , } } }
};
}
