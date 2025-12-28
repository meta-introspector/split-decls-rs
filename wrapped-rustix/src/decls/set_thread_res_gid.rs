macro_rules! deps {
    () => {
        Result!();
        Gid!();
    };
}

macro_rules! set_thread_res_gid {
    () => {
        deps!();
        # [doc = " `setresgid(rgid, egid, sgid)`—Sets the real, effective, and saved group"] # [doc = " ID of the current thread."] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " This is not the `setresgid` you are looking for… POSIX requires gids to be"] # [doc = " process granular, but on Linux they are per-thread. Thus, this call only"] # [doc = " changes the gid for the current *thread*, not the entire process even"] # [doc = " though that is in violation of the POSIX standard."] # [doc = ""] # [doc = " For details on this distinction, see the C library vs. kernel differences"] # [doc = " in the [manual page][linux_notes] and the notes in [`set_thread_gid`]. This"] # [doc = " call implements the kernel behavior."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/setresgid.2.html"] # [doc = " [linux_notes]: https://man7.org/linux/man-pages/man2/setresgid.2.html#NOTES"] # [inline] pub fn set_thread_res_gid < R , E , S > (rgid : R , egid : E , sgid : S) -> io :: Result < () > where R : Into < Option < Gid > > , E : Into < Option < Gid > > , S : Into < Option < Gid > > , { backend :: thread :: syscalls :: setresgid_thread (rgid . into () , egid . into () , sgid . into ()) }
    };
}

set_thread_res_gid!();