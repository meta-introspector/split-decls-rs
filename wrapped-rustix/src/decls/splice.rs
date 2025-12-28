macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! splice {
    () => {
        deps!();
        # [doc = " `splice(fd_in, off_in, fd_out, off_out, len, flags)`—Transfer data"] # [doc = " between a file and a pipe."] # [doc = ""] # [doc = " This function transfers up to `len` bytes of data from the file descriptor"] # [doc = " `fd_in` to the file descriptor `fd_out`, where one of the file descriptors"] # [doc = " must refer to a pipe."] # [doc = ""] # [doc = " `off_*` must be `None` if the corresponding fd refers to a pipe. Otherwise"] # [doc = " its value points to the starting offset to the file, from which the data is"] # [doc = " read/written. On success, the number of bytes read/written is added to the"] # [doc = " offset."] # [doc = ""] # [doc = " Passing `None` causes the read/write to start from the file offset, and the"] # [doc = " file offset is adjusted appropriately."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/splice.2.html"] # [cfg (linux_kernel)] # [inline] pub fn splice < FdIn : AsFd , FdOut : AsFd > (fd_in : FdIn , off_in : Option < & mut u64 > , fd_out : FdOut , off_out : Option < & mut u64 > , len : usize , flags : SpliceFlags ,) -> io :: Result < usize > { backend :: pipe :: syscalls :: splice (fd_in . as_fd () , off_in , fd_out . as_fd () , off_out , len , flags) }
    };
}

splice!();