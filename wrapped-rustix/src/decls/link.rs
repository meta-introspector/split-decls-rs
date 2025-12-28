macro_rules! deps {
    () => {
        Result!();
        Arg!();
    };
}

macro_rules! link {
    () => {
        deps!();
        # [doc = " `link(old_path, new_path)`—Creates a hard link."] # [doc = ""] # [doc = " POSIX leaves it implementation-defined whether `link` follows a symlink in"] # [doc = " `old_path`, or creates a new link to the symbolic link itself. On platforms"] # [doc = " which have it, [`linkat`] avoids this problem since it has an [`AtFlags`]"] # [doc = " parameter and the [`AtFlags::SYMLINK_FOLLOW`] flag determines whether"] # [doc = " symlinks should be followed."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/link.html"] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/link.2.html"] # [doc = " [`linkat`]: crate::fs::linkat"] # [doc = " [`AtFlags`]: crate::fs::AtFlags"] # [doc = " [`AtFlags::SYMLINK_FOLLOW`]: crate::fs::AtFlags::SYMLINK_FOLLOW"] # [inline] pub fn link < P : path :: Arg , Q : path :: Arg > (old_path : P , new_path : Q) -> io :: Result < () > { old_path . into_with_c_str (| old_path | { new_path . into_with_c_str (| new_path | backend :: fs :: syscalls :: link (old_path , new_path)) }) }
    };
}

link!();