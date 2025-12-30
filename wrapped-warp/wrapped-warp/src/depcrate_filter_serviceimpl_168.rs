// Generated macro for impl_168 (impl)
macro_rules! Depcrate_filter_serviceimpl_168 {
() => {
// Module: crate::filter::service
// Provides: {"impl_168"}
// Dependencies: {}
impl < F , B > Service < http :: Request < B > > for FilteredService < F > where F : Filter , < F :: Future as TryFuture > :: Ok : Reply , < F :: Future as TryFuture > :: Error : IsReject , B : http_body :: Body + Send + Sync + 'static , B :: Error : Into < Box < dyn std :: error :: Error + Send + Sync > > , { type Response = Response ; type Error = Infallible ; type Future = FilteredFuture < F :: Future > ; fn poll_ready (& mut self , _ : & mut Context < '_ >) -> Poll < Result < () , Self :: Error > > { Poll :: Ready (Ok (())) } # [inline] fn call (& mut self , req : http :: Request < B >) -> Self :: Future { let req = req . map (crate :: bodyt :: Body :: wrap) ; self . call_route (req) } }
};
}
