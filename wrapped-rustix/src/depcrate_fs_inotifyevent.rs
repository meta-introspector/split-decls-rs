// Generated macro for Event (struct)
macro_rules! Depcrate_fs_inotifyEvent {
() => {
// Module: crate::fs::inotify
// Provides: {"Event"}
// Dependencies: {}
# [doc = " An inotify event."] # [doc (alias = "inotify_event")] # [derive (Debug)] pub struct Event < 'a > { wd : i32 , events : ReadFlags , cookie : u32 , file_name : Option < & 'a CStr > , }
};
}
