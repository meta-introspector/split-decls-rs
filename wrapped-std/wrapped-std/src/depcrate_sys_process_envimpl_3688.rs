// Generated macro for impl_3688 (impl)
macro_rules! Depcrate_sys_process_envimpl_3688 {
() => {
// Module: crate::sys::process::env
// Provides: {"impl_3688"}
// Dependencies: {}
impl CommandEnv { pub fn capture (& self) -> BTreeMap < EnvKey , OsString > { let mut result = BTreeMap :: < EnvKey , OsString > :: new () ; if ! self . clear { for (k , v) in env :: vars_os () { result . insert (k . into () , v) ; } } for (k , maybe_v) in & self . vars { if let & Some (ref v) = maybe_v { result . insert (k . clone () , v . clone ()) ; } else { result . remove (k) ; } } result } pub fn is_unchanged (& self) -> bool { ! self . clear && self . vars . is_empty () } pub fn capture_if_changed (& self) -> Option < BTreeMap < EnvKey , OsString > > { if self . is_unchanged () { None } else { Some (self . capture ()) } } pub fn set (& mut self , key : & OsStr , value : & OsStr) { let key = EnvKey :: from (key) ; self . maybe_saw_path (& key) ; self . vars . insert (key , Some (value . to_owned ())) ; } pub fn remove (& mut self , key : & OsStr) { let key = EnvKey :: from (key) ; self . maybe_saw_path (& key) ; if self . clear { self . vars . remove (& key) ; } else { self . vars . insert (key , None) ; } } pub fn clear (& mut self) { self . clear = true ; self . vars . clear () ; } pub fn does_clear (& self) -> bool { self . clear } pub fn have_changed_path (& self) -> bool { self . saw_path || self . clear } fn maybe_saw_path (& mut self , key : & EnvKey) { if ! self . saw_path && key == "PATH" { self . saw_path = true ; } } pub fn iter (& self) -> CommandEnvs < '_ > { let iter = self . vars . iter () ; CommandEnvs { iter } } }
};
}
