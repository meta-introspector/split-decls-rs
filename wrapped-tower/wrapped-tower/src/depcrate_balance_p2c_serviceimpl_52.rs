// Generated macro for impl_52 (impl)
macro_rules! Depcrate_balance_p2c_serviceimpl_52 {
() => {
// Module: crate::balance::p2c::service
// Provides: {"impl_52"}
// Dependencies: {}
impl < D , Req > Service < Req > for Balance < D , Req > where D : Discover + Unpin , D :: Key : Hash + Clone , D :: Error : Into < crate :: BoxError > , D :: Service : Service < Req > + Load , < D :: Service as Load > :: Metric : std :: fmt :: Debug , < D :: Service as Service < Req > > :: Error : Into < crate :: BoxError > , { type Response = < D :: Service as Service < Req > > :: Response ; type Error = crate :: BoxError ; type Future = future :: MapErr < < D :: Service as Service < Req > > :: Future , fn (< D :: Service as Service < Req > > :: Error) -> crate :: BoxError , > ; fn poll_ready (& mut self , cx : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { let _ = self . update_pending_from_discover (cx) ? ; self . promote_pending_to_ready (cx) ; loop { if let Some (index) = self . ready_index . take () { match self . services . check_ready_index (cx , index) { Ok (true) => { self . ready_index = Some (index) ; return Poll :: Ready (Ok (())) ; } Ok (false) => { trace ! ("ready service became unavailable") ; } Err (Failed (_ , error)) => { debug ! (% error , "endpoint failed") ; } } } self . ready_index = self . p2c_ready_index () ; if self . ready_index . is_none () { debug_assert_eq ! (self . services . ready_len () , 0) ; return Poll :: Pending ; } } } fn call (& mut self , request : Req) -> Self :: Future { let index = self . ready_index . take () . expect ("called before ready") ; self . services . call_ready_index (index , request) . map_err (Into :: into) } }
};
}
