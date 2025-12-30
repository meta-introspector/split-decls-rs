// Generated macro for impl_2213 (impl)
macro_rules! Depcrate_fs_inotifyimpl_2213 {
() => {
// Module: crate::fs::inotify
// Provides: {"impl_2213"}
// Dependencies: {}
impl < 'buf , Fd : AsFd > Reader < 'buf , Fd > { # [doc = " Create a new iterator from the given file descriptor and buffer."] pub fn new (fd : Fd , buf : & 'buf mut [MaybeUninit < u8 >]) -> Self { Self { fd , buf : { let offset = buf . as_ptr () . align_offset (align_of :: < inotify_event > ()) ; if offset < buf . len () { & mut buf [offset ..] } else { & mut [] } } , initialized : 0 , offset : 0 , } } }
};
}
