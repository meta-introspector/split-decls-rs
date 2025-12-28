macro_rules! deps {
    () => {
        LinkNameSpaceType!();
        Result!();
    };
}

macro_rules! move_into_link_name_space {
    () => {
        deps!();
        # [doc = " Reassociate the calling thread with the namespace associated with link"] # [doc = " referred to by `fd`."] # [doc = ""] # [doc = " `fd` must refer to one of the magic links in a `/proc/[pid]/ns/` directory,"] # [doc = " or a bind mount to such a link."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/setns.2.html"] # [doc (alias = "setns")] pub fn move_into_link_name_space (fd : BorrowedFd < '_ > , allowed_type : Option < LinkNameSpaceType > ,) -> io :: Result < () > { let allowed_type = allowed_type . map_or (0 , | t | t as c_int) ; syscalls :: setns (fd , allowed_type) . map (| _r | ()) }
    };
}

move_into_link_name_space!()