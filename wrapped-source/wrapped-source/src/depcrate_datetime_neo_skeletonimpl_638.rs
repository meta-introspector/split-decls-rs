// Generated macro for impl_638 (impl)
macro_rules! Depcrate_datetime_neo_skeletonimpl_638 {
() => {
// Module: crate::datetime::neo_skeleton
// Provides: {"impl_638"}
// Dependencies: {}
impl < 'a > VariantPatterns < 'a > { pub fn iter_in_quality_order_mut (& mut self ,) -> impl Iterator < Item = & mut VariantPatternsElement < 'a > > + '_ { let mut list = [Some (& mut self . standard) , self . variant0 . as_mut () , self . variant1 . as_mut () ,] ; list . sort_by_key (| variant | variant . as_ref () . map (| v | v . distance)) ; list . into_iter () . flatten () } }
};
}
