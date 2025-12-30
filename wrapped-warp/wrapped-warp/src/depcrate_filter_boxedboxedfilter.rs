// Generated macro for BoxedFilter (struct)
macro_rules! Depcrate_filter_boxedBoxedFilter {
() => {
// Module: crate::filter::boxed
// Provides: {"BoxedFilter"}
// Dependencies: {}
# [doc = " A type representing a boxed [`Filter`](crate::Filter) trait object."] # [doc = ""] # [doc = " The filter inside is a dynamic trait object. The purpose of this type is"] # [doc = " to ease returning `Filter`s from other functions."] # [doc = ""] # [doc = " To create one, call `Filter::boxed` on any filter."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use warp::{Filter, filters::BoxedFilter, Reply};"] # [doc = ""] # [doc = " pub fn assets_filter() -> BoxedFilter<(impl Reply,)> {"] # [doc = "     warp::path(\"assets\")"] # [doc = "         .and(warp::fs::dir(\"./assets\"))"] # [doc = "         .boxed()"] # [doc = " }"] # [doc = " ```"] # [doc = ""] pub struct BoxedFilter < T : Tuple > { filter : Arc < dyn Filter < Extract = T , Error = Rejection , Future = Pin < Box < dyn Future < Output = Result < T , Rejection > > + Send > > , > + Send + Sync , > , }
};
}
