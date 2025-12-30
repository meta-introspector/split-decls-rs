// Generated macro for impl_167 (impl)
macro_rules! Depcrate_filter_serviceimpl_167 {
() => {
// Module: crate::filter::service
// Provides: {"impl_167"}
// Dependencies: {}
impl < F > FilteredService < F > where F : Filter , < F :: Future as TryFuture > :: Ok : Reply , < F :: Future as TryFuture > :: Error : IsReject , { # [inline] pub (crate) fn call_route (& self , req : Request) -> FilteredFuture < F :: Future > { debug_assert ! (! route :: is_set () , "nested route::set calls") ; let route = Route :: new (req) ; let fut = route :: set (& route , | | self . filter . filter (super :: Internal)) ; FilteredFuture { future : fut , route } } }
};
}
