// Generated macro for ext4_ioc_resize_fs (function)
macro_rules! Depcrate_fs_ioctlext4_ioc_resize_fs {
() => {
// Module: crate::fs::ioctl
// Provides: {"ext4_ioc_resize_fs"}
// Dependencies: {}
# [doc = " `ioctl(fd, EXT4_IOC_RESIZE_FS, blocks)`—Resize ext4 filesystem on fd."] # [cfg (linux_raw_dep)] # [inline] # [doc (alias = "EXT4_IOC_RESIZE_FS")] pub fn ext4_ioc_resize_fs < Fd : AsFd > (fd : Fd , blocks : u64) -> io :: Result < () > { unsafe { let ctl = ioctl :: Setter :: < { backend :: fs :: EXT4_IOC_RESIZE_FS } , u64 > :: new (blocks) ; ioctl :: ioctl (fd , ctl) } }
};
}
