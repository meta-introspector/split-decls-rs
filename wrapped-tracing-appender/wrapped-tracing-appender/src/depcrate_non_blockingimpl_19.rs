// Generated macro for impl_19 (impl)
macro_rules! Depcrate_non_blockingimpl_19 {
() => {
// Module: crate::non_blocking
// Provides: {"impl_19"}
// Dependencies: {}
impl NonBlocking { # [doc = " Returns a new `NonBlocking` writer wrapping the provided `writer`."] # [doc = ""] # [doc = " The returned `NonBlocking` writer will have the [default configuration][default] values."] # [doc = " Other configurations can be specified using the [builder] interface."] # [doc = ""] # [doc = " [default]: NonBlockingBuilder::default"] # [doc = " [builder]: NonBlockingBuilder"] pub fn new < T : Write + Send + 'static > (writer : T) -> (NonBlocking , WorkerGuard) { NonBlockingBuilder :: default () . finish (writer) } fn create < T : Write + Send + 'static > (writer : T , buffered_lines_limit : usize , is_lossy : bool , thread_name : String ,) -> (NonBlocking , WorkerGuard) { let (sender , receiver) = bounded (buffered_lines_limit) ; let (shutdown_sender , shutdown_receiver) = bounded (0) ; let worker = Worker :: new (receiver , writer , shutdown_receiver) ; let worker_guard = WorkerGuard :: new (worker . worker_thread (thread_name) , sender . clone () , shutdown_sender ,) ; (Self { channel : sender , error_counter : ErrorCounter (Arc :: new (AtomicUsize :: new (0))) , is_lossy , } , worker_guard ,) } # [doc = " Returns a counter for the number of times logs where dropped. This will always return zero if"] # [doc = " `NonBlocking` is not lossy."] pub fn error_counter (& self) -> ErrorCounter { self . error_counter . clone () } }
};
}
