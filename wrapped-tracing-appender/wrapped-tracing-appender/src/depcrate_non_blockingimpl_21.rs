// Generated macro for impl_21 (impl)
macro_rules! Depcrate_non_blockingimpl_21 {
() => {
// Module: crate::non_blocking
// Provides: {"impl_21"}
// Dependencies: {}
impl NonBlockingBuilder { # [doc = " Sets the number of lines to buffer before dropping logs or exerting backpressure on senders"] pub fn buffered_lines_limit (mut self , buffered_lines_limit : usize) -> NonBlockingBuilder { self . buffered_lines_limit = buffered_lines_limit ; self } # [doc = " Sets whether `NonBlocking` should be lossy or not."] # [doc = ""] # [doc = " If set to `true`, logs will be dropped when the buffered limit is reached. If `false`, backpressure"] # [doc = " will be exerted on senders, blocking them until the buffer has capacity again."] # [doc = ""] # [doc = " By default, the built `NonBlocking` will be lossy."] pub fn lossy (mut self , is_lossy : bool) -> NonBlockingBuilder { self . is_lossy = is_lossy ; self } # [doc = " Override the worker thread's name."] # [doc = ""] # [doc = " The default worker thread name is \"tracing-appender\"."] pub fn thread_name (mut self , name : & str) -> NonBlockingBuilder { self . thread_name = name . to_string () ; self } # [doc = " Completes the builder, returning the configured `NonBlocking`."] pub fn finish < T : Write + Send + 'static > (self , writer : T) -> (NonBlocking , WorkerGuard) { NonBlocking :: create (writer , self . buffered_lines_limit , self . is_lossy , self . thread_name ,) } }
};
}
