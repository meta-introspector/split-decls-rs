macro_rules! fd {
    () => {
        # [doc = " Export the `*Fd` types and traits that are used in rustix's public API."] # [doc = ""] # [doc = " This module exports the types and traits from [`std::os::fd`], or polyills"] # [doc = " on Rust < 1.66 or on Windows."] # [doc = ""] # [doc = " On Windows, the polyfill consists of aliases of the socket types and"] # [doc = " traits, For example, [`OwnedSocket`] is aliased to `OwnedFd`, and so on,"] # [doc = " and there are blanket impls for `AsFd` etc. that map to `AsSocket` impls."] # [doc = " These blanket impls suffice for using the traits, however not for"] # [doc = " implementing them, so this module also exports `AsSocket` and the other"] # [doc = " traits as-is so that users can implement them if needed."] # [doc = ""] # [doc = " [`OwnedSocket`]: https://doc.rust-lang.org/stable/std/os/windows/io/struct.OwnedSocket.html"] pub mod fd { pub use super :: backend :: fd :: * ; }
    };
}

fd!()