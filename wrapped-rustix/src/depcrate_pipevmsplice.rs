// Generated macro for vmsplice (function)
macro_rules! Depcrate_pipevmsplice {
() => {
// Module: crate::pipe
// Provides: {"vmsplice"}
// Dependencies: {}
# [doc = " `vmsplice(fd, bufs, flags)`—Transfer data between memory and a pipe."] # [doc = ""] # [doc = " If `fd` is the write end of the pipe, the function maps the memory pointer"] # [doc = " at by `bufs` to the pipe."] # [doc = ""] # [doc = " If `fd` is the read end of the pipe, the function writes data from the pipe"] # [doc = " to said memory."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " If the memory must not be mutated (such as when `bufs` were originally"] # [doc = " immutable slices), it is up to the caller to ensure that the write end of"] # [doc = " the pipe is placed in `fd`."] # [doc = ""] # [doc = " Additionally if `SpliceFlags::GIFT` is set, the caller must also ensure"] # [doc = " that the contents of `bufs` in never modified following the call, and that"] # [doc = " all of the pointers in `bufs` are page aligned, and the lengths are"] # [doc = " multiples of a page size in bytes."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/vmsplice.2.html"] # [cfg (linux_kernel)] # [inline] pub unsafe fn vmsplice < PipeFd : AsFd > (fd : PipeFd , bufs : & [IoSliceRaw < '_ >] , flags : SpliceFlags ,) -> io :: Result < usize > { backend :: pipe :: syscalls :: vmsplice (fd . as_fd () , bufs , flags) }
};
}
