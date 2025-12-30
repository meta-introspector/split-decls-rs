// Generated macro for impl_1762 (impl)
macro_rules! Depcrate_os_wasi_fsimpl_1762 {
() => {
// Module: crate::os::wasi::fs
// Provides: {"impl_1762"}
// Dependencies: {}
impl OpenOptionsExt for OpenOptions { fn lookup_flags (& mut self , flags : u32) -> & mut OpenOptions { self . as_inner_mut () . lookup_flags (flags) ; self } fn directory (& mut self , dir : bool) -> & mut OpenOptions { self . as_inner_mut () . directory (dir) ; self } fn dsync (& mut self , enabled : bool) -> & mut OpenOptions { self . as_inner_mut () . dsync (enabled) ; self } fn nonblock (& mut self , enabled : bool) -> & mut OpenOptions { self . as_inner_mut () . nonblock (enabled) ; self } fn rsync (& mut self , enabled : bool) -> & mut OpenOptions { self . as_inner_mut () . rsync (enabled) ; self } fn sync (& mut self , enabled : bool) -> & mut OpenOptions { self . as_inner_mut () . sync (enabled) ; self } fn fs_rights_base (& mut self , rights : u64) -> & mut OpenOptions { self . as_inner_mut () . fs_rights_base (rights) ; self } fn fs_rights_inheriting (& mut self , rights : u64) -> & mut OpenOptions { self . as_inner_mut () . fs_rights_inheriting (rights) ; self } fn open_at < P : AsRef < Path > > (& self , file : & File , path : P) -> io :: Result < File > { let inner = file . as_inner () . open_at (path . as_ref () , self . as_inner ()) ? ; Ok (File :: from_inner (inner)) } }
};
}
