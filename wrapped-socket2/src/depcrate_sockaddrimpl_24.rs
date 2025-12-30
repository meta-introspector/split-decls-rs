// Generated macro for impl_24 (impl)
macro_rules! Depcrate_sockaddrimpl_24 {
() => {
// Module: crate::sockaddr
// Provides: {"impl_24"}
// Dependencies: {}
impl SockAddrStorage { # [doc = " Construct a new storage containing all zeros."] # [inline] pub fn zeroed () -> Self { unsafe { mem :: zeroed () } } # [doc = " Returns the size of this storage."] # [inline] pub fn size_of (& self) -> socklen_t { size_of :: < Self > () as socklen_t } # [doc = " View this type as another type."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The type `T` must be one of the `sockaddr_*` types defined by this platform."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " # #[allow(dead_code)]"] # [doc = " # #[cfg(unix)] mod unix_example {"] # [doc = " # use core::mem::size_of;"] # [doc = " use libc::sockaddr_storage;"] # [doc = " use socket2::{SockAddr, SockAddrStorage, socklen_t};"] # [doc = ""] # [doc = " fn from_sockaddr_storage(recv_address: &sockaddr_storage) -> SockAddr {"] # [doc = "     let mut storage = SockAddrStorage::zeroed();"] # [doc = "     let libc_address = unsafe { storage.view_as::<sockaddr_storage>() };"] # [doc = "     *libc_address = *recv_address;"] # [doc = "     unsafe { SockAddr::new(storage, size_of::<sockaddr_storage>() as socklen_t) }"] # [doc = " }"] # [doc = " # }"] # [doc = " ```"] # [inline] pub unsafe fn view_as < T > (& mut self) -> & mut T { assert ! (size_of ::< T > () <= size_of ::< Self > ()) ; & mut * (self as * mut Self as * mut T) } }
};
}
