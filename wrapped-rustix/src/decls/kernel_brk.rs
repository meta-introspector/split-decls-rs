macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! kernel_brk {
    () => {
        deps!();
        # [doc = " `brk(addr)`—Change the location of the “program break”."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " This is not identical to `brk` in libc. libc `brk` may have bookkeeping"] # [doc = " that needs to be kept up to date that this doesn't keep up to date, so"] # [doc = " don't use it unless you know your code won't share a process with a libc"] # [doc = " (perhaps because you yourself are implementing a libc)."] # [inline] pub unsafe fn kernel_brk (addr : * mut c_void) -> io :: Result < * mut c_void > { backend :: runtime :: syscalls :: kernel_brk (addr) }
    };
}

kernel_brk!();