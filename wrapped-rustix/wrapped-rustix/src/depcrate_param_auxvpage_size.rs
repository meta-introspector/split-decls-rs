// Generated macro for page_size (function)
macro_rules! Depcrate_param_auxvpage_size {
() => {
// Module: crate::param::auxv
// Provides: {"page_size"}
// Dependencies: {}
# [doc = " `sysconf(_SC_PAGESIZE)`—Returns the process' page size."] # [doc = ""] # [doc = " Also known as `getpagesize`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [POSIX]"] # [doc = "  - [Linux `sysconf`]"] # [doc = "  - [Linux `getpagesize`]"] # [doc = ""] # [doc = " [POSIX]: https://pubs.opengroup.org/onlinepubs/9799919799/functions/sysconf.html"] # [doc = " [Linux `sysconf`]: https://man7.org/linux/man-pages/man3/sysconf.3.html"] # [doc = " [Linux `getpagesize`]: https://man7.org/linux/man-pages/man2/getpagesize.2.html"] # [inline] # [doc (alias = "PAGESIZE")] # [doc (alias = "PAGE_SIZE")] # [doc (alias = "_SC_PAGESIZE")] # [doc (alias = "_SC_PAGE_SIZE")] # [doc (alias = "getpagesize")] pub fn page_size () -> usize { backend :: param :: auxv :: page_size () }
};
}
