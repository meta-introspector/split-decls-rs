// Generated macro for impl_2215 (impl)
macro_rules! Depcrate_fs_inotifyimpl_2215 {
() => {
// Module: crate::fs::inotify
// Provides: {"impl_2215"}
// Dependencies: {}
impl < 'a > Event < 'a > { # [doc = " Returns the watch for which this event occurs."] # [inline] pub fn wd (& self) -> i32 { self . wd } # [doc = " Returns a description of the events."] # [inline] # [doc (alias = "mask")] pub fn events (& self) -> ReadFlags { self . events } # [doc = " Returns the unique cookie associating related events."] # [inline] pub fn cookie (& self) -> u32 { self . cookie } # [doc = " Returns the file name of this event, if any."] # [inline] pub fn file_name (& self) -> Option < & CStr > { self . file_name } }
};
}
