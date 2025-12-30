// Generated macro for poll_handle_early_data (function)
macro_rules! Depcrate_clientpoll_handle_early_data {
() => {
// Module: crate::client
// Provides: {"poll_handle_early_data"}
// Dependencies: {}
# [cfg (feature = "early-data")] fn poll_handle_early_data < IO > (state : & mut TlsState , stream : & mut Stream < IO , ClientConnection > , early_waker : & mut Option < Waker > , cx : & mut Context < '_ > , bufs : & [io :: IoSlice < '_ >] ,) -> Poll < io :: Result < usize > > where IO : AsyncRead + AsyncWrite + Unpin , { if let TlsState :: EarlyData (pos , data) = state { use std :: io :: Write ; if let Some (mut early_data) = stream . session . early_data () { let mut written = 0 ; for buf in bufs { if buf . is_empty () { continue ; } let len = match early_data . write (buf) { Ok (0) => break , Ok (n) => n , Err (err) => return Poll :: Ready (Err (err)) , } ; written += len ; data . extend_from_slice (& buf [.. len]) ; if len < buf . len () { break ; } } if written != 0 { return Poll :: Ready (Ok (written)) ; } } while stream . session . is_handshaking () { ready ! (stream . handshake (cx)) ? ; } if ! stream . session . is_early_data_accepted () { while * pos < data . len () { let len = ready ! (stream . as_mut_pin () . poll_write (cx , & data [* pos ..])) ? ; * pos += len ; } } * state = TlsState :: Stream ; if let Some (waker) = early_waker . take () { waker . wake () ; } } Poll :: Ready (Ok (0)) }
};
}
