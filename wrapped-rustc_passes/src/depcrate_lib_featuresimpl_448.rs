// Generated macro for impl_448 (impl)
macro_rules! Depcrate_lib_featuresimpl_448 {
() => {
// Module: crate::lib_features
// Provides: {"impl_448"}
// Dependencies: {}
impl < 'tcx > Visitor < 'tcx > for LibFeatureCollector < 'tcx > { type NestedFilter = nested_filter :: All ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_attribute (& mut self , attr : & 'tcx Attribute) { if let Some ((feature , stable , span)) = self . extract (attr) { self . collect_feature (feature , stable , span) ; } } }
};
}
