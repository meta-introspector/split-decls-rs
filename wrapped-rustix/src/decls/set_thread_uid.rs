macro_rules! deps {
    () => {
        Result!();
        Uid!();
    };
}

macro_rules! set_thread_uid {
    () => {
        deps!();
        # [doc = " `setuid(uid)`—Sets the effective user ID of the calling thread."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " This is not the `setuid` you are looking for… POSIX requires uids to be"] # [doc = " process granular, but on Linux they are per-thread. Thus, this call only"] # [doc = " changes the uid for the current *thread*, not the entire process even"] # [doc = " though that is in violation of the POSIX standard."] # [doc = ""] # [doc = " For details on this distinction, see the C library vs. kernel differences"] # [doc = " in the [manual page][linux_notes]. This call implements the kernel"] # [doc = " behavior."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/setuid.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/setuid.2.html"] # [doc = " [linux_notes]: https://man7.org/linux/man-pages/man2/setuid.2.html#NOTES"] # [inline] pub fn set_thread_uid (uid : Uid) -> io :: Result < () > { backend :: thread :: syscalls :: setuid_thread (uid) }
    };
}

set_thread_uid!()