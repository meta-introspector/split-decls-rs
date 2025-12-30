// Generated macro for impl_13 (impl)
macro_rules! Depcrate_pointimpl_13 {
() => {
// Module: crate::point
// Provides: {"impl_13"}
// Dependencies: {}
impl < T > ModulusSize for T where T : 'static + ArraySize + Copy + Debug , T : Add < U1 , Output : 'static + ArraySize + Copy + Debug > , T : Add < T , Output : 'static + ArraySize + Copy + Debug + Sub < T , Output = T > > , < T as Add < U1 > > :: Output : Add < T , Output : 'static + ArraySize + Copy + Debug > , { type CompressedPointSize = < T as Add < U1 > > :: Output ; type UncompressedPointSize = < Self :: CompressedPointSize as Add < T > > :: Output ; type UntaggedPointSize = < T as Add < T > > :: Output ; }
};
}
