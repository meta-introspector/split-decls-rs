macro_rules! deps {
    () => {
        Gid!();
        Result!();
    };
}

macro_rules! set_thread_groups {
    () => {
        deps!();
        # [doc = " `setgroups(groups)`—Sets the supplementary group IDs for the calling"] # [doc = " thread."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " This is not the `setgroups` you are looking for… POSIX requires gids to be"] # [doc = " process granular, but on Linux they are per-thread. Thus, this call only"] # [doc = " changes the gids for the current *thread*, not the entire process even"] # [doc = " though that is in violation of the POSIX standard."] # [doc = ""] # [doc = " For details on this distinction, see the C library vs. kernel differences"] # [doc = " in the [manual page][linux_notes]. This call implements the kernel"] # [doc = " behavior."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/setgroups.2.html"] # [doc = " [linux_notes]: https://man7.org/linux/man-pages/man2/setgroups.2.html#NOTES"] # [cfg (linux_kernel)] # [inline] pub fn set_thread_groups (groups : & [Gid]) -> io :: Result < () > { backend :: thread :: syscalls :: setgroups_thread (groups) }
    };
}

set_thread_groups!()