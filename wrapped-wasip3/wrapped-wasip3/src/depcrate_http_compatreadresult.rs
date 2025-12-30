// Generated macro for ReadResult (enum)
macro_rules! Depcrate_http_compatReadResult {
() => {
// Module: crate::http_compat
// Provides: {"ReadResult"}
// Dependencies: {}
enum ReadResult { Trailers (Result < Option < types :: Trailers > , ErrorCode >) , BodyChunk { chunk : Vec < u8 > , stream : wit_bindgen :: StreamReader < u8 > , trailers : wit_bindgen :: FutureReader < Result < Option < types :: Trailers > , ErrorCode > > , } , }
};
}
