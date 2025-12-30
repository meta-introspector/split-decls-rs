// Generated macro for non_blocking (function)
macro_rules! Depcratenon_blocking {
() => {
// Module: crate
// Provides: {"non_blocking"}
// Dependencies: {}
# [doc = " Convenience function for creating a non-blocking, off-thread writer."] # [doc = ""] # [doc = " See the [`non_blocking` module's docs][non_blocking]'s for more details."] # [doc = ""] # [doc = " [non_blocking]: mod@non_blocking"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ``` rust"] # [doc = " # fn docs() {"] # [doc = " let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());"] # [doc = " let subscriber = tracing_subscriber::fmt().with_writer(non_blocking);"] # [doc = " tracing::subscriber::with_default(subscriber.finish(), || {"] # [doc = "    tracing::event!(tracing::Level::INFO, \"Hello\");"] # [doc = " });"] # [doc = " # }"] # [doc = " ```"] pub fn non_blocking < T : Write + Send + 'static > (writer : T) -> (NonBlocking , WorkerGuard) { NonBlocking :: new (writer) }
};
}
