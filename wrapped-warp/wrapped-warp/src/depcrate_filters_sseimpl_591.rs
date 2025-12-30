// Generated macro for impl_591 (impl)
macro_rules! Depcrate_filters_sseimpl_591 {
() => {
// Module: crate::filters::sse
// Provides: {"impl_591"}
// Dependencies: {}
impl KeepAlive { # [doc = " Customize the interval between keep-alive messages."] # [doc = ""] # [doc = " Default is 15 seconds."] pub fn interval (mut self , time : Duration) -> Self { self . max_interval = time ; self } # [doc = " Customize the text of the keep-alive message."] # [doc = ""] # [doc = " Default is an empty comment."] pub fn text (mut self , text : impl Into < Cow < 'static , str > >) -> Self { self . comment_text = text . into () ; self } # [doc = " Wrap an event stream with keep-alive functionality."] # [doc = ""] # [doc = " See [`keep_alive`] for more."] pub fn stream < S > (self , event_stream : S ,) -> impl TryStream < Ok = Event , Error = impl StdError + Send + Sync + 'static > + Send + 'static where S : TryStream < Ok = Event > + Send + 'static , S :: Error : StdError + Send + Sync + 'static , { let alive_timer = time :: sleep (self . max_interval) ; SseKeepAlive { event_stream , comment_text : self . comment_text , max_interval : self . max_interval , alive_timer , } } }
};
}
