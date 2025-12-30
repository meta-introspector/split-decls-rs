// Generated macro for NotifyActor (struct)
macro_rules! DepcrateNotifyActor {
() => {
// Module: crate
// Provides: {"NotifyActor"}
// Dependencies: {}
struct NotifyActor { sender : loader :: Sender , watched_file_entries : FxHashSet < AbsPathBuf > , watched_dir_entries : Vec < loader :: Directories > , watcher : Option < (RecommendedWatcher , Receiver < NotifyEvent >) > , }
};
}
