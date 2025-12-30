// Generated macro for BackgroundTask (struct)
macro_rules! DepcrateBackgroundTask {
() => {
// Module: crate
// Provides: {"BackgroundTask"}
// Dependencies: {}
# [doc = " The background task that ships logs to Loki. It must be [`tokio::spawn`]ed"] # [doc = " by the calling application."] # [doc = ""] # [doc = " See the crate's root documentation for an example."] pub struct BackgroundTask { loki_url : Url , receiver : mpsc :: Receiver < Option < LokiEvent > > , queues : LevelMap < SendQueue > , buffer : Buffer , http_client : reqwest :: Client , backoff_count : u32 , backoff : Option < Pin < Box < tokio :: time :: Sleep > > > , quitting : bool , send_task : Option < Pin < Box < dyn Future < Output = Result < () , Box < dyn error :: Error > > > + Send + 'static > > > , }
};
}
