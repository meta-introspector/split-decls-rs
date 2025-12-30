// Generated macro for impl_594 (impl)
macro_rules! Depcrate_filters_sseimpl_594 {
() => {
// Module: crate::filters::sse
// Provides: {"impl_594"}
// Dependencies: {}
impl < S > Stream for SseKeepAlive < S > where S : TryStream < Ok = Event > + Send + 'static , S :: Error : StdError + Send + Sync + 'static , { type Item = Result < Event , SseError > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut pin = self . project () ; match pin . event_stream . try_poll_next (cx) { Poll :: Pending => match Pin :: new (& mut pin . alive_timer) . poll (cx) { Poll :: Pending => Poll :: Pending , Poll :: Ready (_) => { pin . alive_timer . reset (tokio :: time :: Instant :: now () + * pin . max_interval) ; let comment_str = pin . comment_text . clone () ; let event = Event :: default () . comment (comment_str) ; Poll :: Ready (Some (Ok (event))) } } , Poll :: Ready (Some (Ok (event))) => { pin . alive_timer . reset (tokio :: time :: Instant :: now () + * pin . max_interval) ; Poll :: Ready (Some (Ok (event))) } Poll :: Ready (None) => Poll :: Ready (None) , Poll :: Ready (Some (Err (error))) => { log :: error ! ("sse::keep error: {}" , error) ; Poll :: Ready (Some (Err (SseError))) } } } }
};
}
