// Generated macro for impl_763 (impl)
macro_rules! Depcrate_util_call_all_commonimpl_763 {
() => {
// Module: crate::util::call_all::common
// Provides: {"impl_763"}
// Dependencies: {}
impl < Svc , S , Q > CallAll < Svc , S , Q > where Svc : Service < S :: Item > , S : Stream , Q : Drive < Svc :: Future > , { pub (crate) const fn new (service : Svc , stream : S , queue : Q) -> CallAll < Svc , S , Q > { CallAll { service : Some (service) , stream , queue , eof : false , curr_req : None , } } # [doc = " Extract the wrapped [`Service`]."] pub (crate) fn into_inner (mut self) -> Svc { self . service . take () . expect ("Service already taken") } # [doc = " Extract the wrapped [`Service`]."] pub (crate) fn take_service (self : Pin < & mut Self >) -> Svc { self . project () . service . take () . expect ("Service already taken") } pub (crate) fn unordered (mut self) -> super :: CallAllUnordered < Svc , S > { assert ! (self . queue . is_empty () && ! self . eof) ; super :: CallAllUnordered :: new (self . service . take () . unwrap () , self . stream) } }
};
}
