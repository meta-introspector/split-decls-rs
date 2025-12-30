// Generated macro for BackgroundTaskController (struct)
macro_rules! DepcrateBackgroundTaskController {
() => {
// Module: crate
// Provides: {"BackgroundTaskController"}
// Dependencies: {}
# [doc = " Handle to cleanly shut down the `BackgroundTask`."] # [doc = ""] # [doc = " It'll still try to send all available data and then quit."] pub struct BackgroundTaskController { sender : mpsc :: Sender < Option < LokiEvent > > , }
};
}
