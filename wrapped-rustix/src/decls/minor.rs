macro_rules! minor {
    () => {
        # [doc = " `minor(dev)`—Compute the minor ID of a given device ID."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man3/minor.3.html"] # [inline] pub fn minor (dev : Dev) -> u32 { backend :: fs :: makedev :: minor (dev) }
    };
}

minor!()