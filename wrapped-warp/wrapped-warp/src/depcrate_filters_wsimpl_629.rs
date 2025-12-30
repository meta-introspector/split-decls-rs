// Generated macro for impl_629 (impl)
macro_rules! Depcrate_filters_wsimpl_629 {
() => {
// Module: crate::filters::ws
// Provides: {"impl_629"}
// Dependencies: {}
impl Ws { # [doc = " Finish the upgrade, passing a function to handle the `WebSocket`."] # [doc = ""] # [doc = " The passed function must return a `Future`."] pub fn on_upgrade < F , U > (self , func : F) -> impl Reply where F : FnOnce (WebSocket) -> U + Send + 'static , U : Future < Output = () > + Send + 'static , { WsReply { ws : self , on_upgrade : func , } } # [doc = " Does nothing."] # [doc = ""] # [doc = " # Deprecated"] # [doc = ""] # [doc = " Use `max_write_buffer_size()` instead."] # [deprecated = "use max_write_buffer_size instead"] pub fn max_send_queue (self , _max : usize) -> Self { self } # [doc = " The max size of the write buffer, in bytes."] pub fn max_write_buffer_size (mut self , max : usize) -> Self { self . config . get_or_insert_with (WebSocketConfig :: default) . max_write_buffer_size = max ; self } # [doc = " Set the maximum message size (defaults to 64 megabytes)"] pub fn max_message_size (mut self , max : usize) -> Self { self . config . get_or_insert_with (WebSocketConfig :: default) . max_message_size = Some (max) ; self } # [doc = " Set the maximum frame size (defaults to 16 megabytes)"] pub fn max_frame_size (mut self , max : usize) -> Self { self . config . get_or_insert_with (WebSocketConfig :: default) . max_frame_size = Some (max) ; self } }
};
}
