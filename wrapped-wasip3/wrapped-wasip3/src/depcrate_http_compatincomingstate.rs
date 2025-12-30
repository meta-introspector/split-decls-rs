// Generated macro for IncomingState (enum)
macro_rules! Depcrate_http_compatIncomingState {
() => {
// Module: crate::http_compat
// Provides: {"IncomingState"}
// Dependencies: {}
enum IncomingState { Ready { stream : wit_bindgen :: StreamReader < u8 > , trailers : wit_bindgen :: FutureReader < Result < Option < types :: Trailers > , ErrorCode > > , } , Reading (Pin < Box < dyn std :: future :: Future < Output = ReadResult > + 'static + Send > >) , Done , }
};
}
