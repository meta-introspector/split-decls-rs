use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct NotifyActor { sender : loader :: Sender , watched_file_entries : FxHashSet < AbsPathBuf > , watched_dir_entries : Vec < loader :: Directories > , watcher : Option < (RecommendedWatcher , Receiver < NotifyEvent >) > , }
}