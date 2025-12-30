// Generated macro for impl_240 (impl)
macro_rules! Depcrate_impls_initializedimpl_240 {
() => {
// Module: crate::impls::initialized
// Provides: {"impl_240"}
// Dependencies: {}
impl < 'a , 'tcx > MaybeInitializedPlaces < 'a , 'tcx > { fn update_bits (state : & mut < Self as Analysis < 'tcx > > :: Domain , path : MovePathIndex , dfstate : DropFlagState ,) { match dfstate { DropFlagState :: Absent => state . kill (path) , DropFlagState :: Present => state . gen_ (path) , } } }
};
}
