macro_rules! Direction {
    () => {
        # [doc = " The direction that an `ioctl` is going."] # [doc = ""] # [doc = " The direction is relative to userspace: `Read` means reading data from the"] # [doc = " kernel, and `Write` means the kernel writing data to userspace."] # [derive (Debug , Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash)] pub enum Direction { # [doc = " None of the above."] None , # [doc = " Read data from the kernel."] Read , # [doc = " Write data to the kernel."] Write , # [doc = " Read and write data to the kernel."] ReadWrite , }
    };
}

Direction!();