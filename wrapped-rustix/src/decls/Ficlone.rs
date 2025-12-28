macro_rules! Ficlone {
    () => {
        # [cfg (all (linux_kernel , not (any (target_arch = "sparc" , target_arch = "sparc64"))))] struct Ficlone < 'a > (BorrowedFd < 'a >) ;
    };
}

Ficlone!();