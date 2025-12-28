macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! fallocate {
    () => {
        deps!();
        # [doc = " `fallocate(fd, mode, offset, len)`—Adjusts file allocation."] # [doc = ""] # [doc = " This is a more general form of `posix_fallocate`, adding a `mode` argument"] # [doc = " which modifies the behavior. On platforms which only support"] # [doc = " `posix_fallocate` and not the more general form, no `FallocateFlags` values"] # [doc = " are defined so it will always be empty."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux `fallocate`]"] # [doc = "  - [Linux `posix_fallocate`]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/posix_fallocate.html"] # [doc = " [Linux `fallocate`]: https://man7.org/linux/man-pages/man2/fallocate.2.html"] # [doc = " [Linux `posix_fallocate`]: https://man7.org/linux/man-pages/man3/posix_fallocate.3.html"] # [cfg (not (any (netbsdlike , target_os = "dragonfly" , target_os = "espidf" , target_os = "horizon" , target_os = "nto" , target_os = "redox" , target_os = "vita" ,)))] # [inline] # [doc (alias = "posix_fallocate")] pub fn fallocate < Fd : AsFd > (fd : Fd , mode : FallocateFlags , offset : u64 , len : u64) -> io :: Result < () > { backend :: fs :: syscalls :: fallocate (fd . as_fd () , mode , offset , len) }
    };
}

fallocate!();