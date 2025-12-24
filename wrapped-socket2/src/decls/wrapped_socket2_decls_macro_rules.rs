use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Link to online documentation for (almost) all supported OSs.
#[rustfmt::skip]
macro_rules! man_links {
    ($syscall:tt ($section:tt)) => {
        concat!(man_links!(__ intro), man_links!(__ unix $syscall ($section)),
        man_links!(__ windows $syscall ($section)),)
    };
    (unix : $syscall:tt ($section:tt)) => {
        concat!(man_links!(__ intro), man_links!(__ unix $syscall ($section)),)
    };
    (windows : $syscall:tt ($section:tt)) => {
        concat!(man_links!(__ intro), man_links!(__ windows $syscall ($section)),)
    };
    (__ intro) => {
        "\n\nAdditional documentation can be found in manual of the OS:\n\n"
    };
    (__ unix $syscall:tt ($section:tt)) => {
        concat!(" * DragonFly BSD: <https://man.dragonflybsd.org/?command=",
        stringify!($syscall), "&section=", stringify!($section), ">\n",
        " * FreeBSD: <https://www.freebsd.org/cgi/man.cgi?query=", stringify!($syscall),
        "&sektion=", stringify!($section), ">\n",
        " * Linux: <https://man7.org/linux/man-pages/man", stringify!($section), "/",
        stringify!($syscall), ".", stringify!($section), ".html>\n",
        " * macOS: <https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/",
        stringify!($syscall), ".", stringify!($section),
        ".html> (archived, actually for iOS)\n", " * NetBSD: <https://man.netbsd.org/",
        stringify!($syscall), ".", stringify!($section), ">\n",
        " * OpenBSD: <https://man.openbsd.org/", stringify!($syscall), ".",
        stringify!($section), ">\n",
        " * iOS: <https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man2/",
        stringify!($syscall), ".", stringify!($section), ".html> (archived)\n",
        " * illumos: <https://illumos.org/man/3SOCKET/", stringify!($syscall), ">\n",)
    };
    (__ windows $syscall:tt ($section:tt)) => {
        concat!(" * Windows: <https://docs.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-",
        stringify!($syscall), ">\n",)
    };
}
