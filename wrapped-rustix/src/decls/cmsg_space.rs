macro_rules! deps {
    () => {
        UCred!();
    };
}

macro_rules! cmsg_space {
    () => {
        deps!();
        # [doc = " Macro for defining the amount of space to allocate in a buffer for use with"] # [doc = " [`RecvAncillaryBuffer::new`] and [`SendAncillaryBuffer::new`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Allocate a buffer for a single file descriptor:"] # [doc = " ```"] # [doc = " # use std::mem::MaybeUninit;"] # [doc = " # use rustix::cmsg_space;"] # [doc = " let mut space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(1))];"] # [doc = " # let _: &[MaybeUninit<u8>] = space.as_slice();"] # [doc = " ```"] # [doc = ""] # [doc = " Allocate a buffer for credentials:"] # [doc = " ```"] # [doc = " # #[cfg(linux_kernel)]"] # [doc = " # {"] # [doc = " # use std::mem::MaybeUninit;"] # [doc = " # use rustix::cmsg_space;"] # [doc = " let mut space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmCredentials(1))];"] # [doc = " # let _: &[MaybeUninit<u8>] = space.as_slice();"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " Allocate a buffer for two file descriptors and credentials:"] # [doc = " ```"] # [doc = " # #[cfg(linux_kernel)]"] # [doc = " # {"] # [doc = " # use std::mem::MaybeUninit;"] # [doc = " # use rustix::cmsg_space;"] # [doc = " let mut space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(2), ScmCredentials(1))];"] # [doc = " # let _: &[MaybeUninit<u8>] = space.as_slice();"] # [doc = " # }"] # [doc = " ```"] # [macro_export] macro_rules ! cmsg_space { (ScmRights ($ len : expr)) => { $ crate :: net :: __cmsg_space ($ len * :: core :: mem :: size_of ::<$ crate :: fd :: BorrowedFd <'static >> () ,) } ; (ScmCredentials ($ len : expr)) => { $ crate :: net :: __cmsg_space ($ len * :: core :: mem :: size_of ::<$ crate :: net :: UCred > () ,) } ; (TxTime ($ len : expr)) => { $ crate :: net :: __cmsg_space ($ len * :: core :: mem :: size_of ::<:: core :: primitive :: u64 > () ,) } ; ($ firstid : ident ($ firstex : expr) , $ ($ restid : ident ($ restex : expr)) ,*) => { { let sum = $ crate :: cmsg_space ! ($ firstid ($ firstex)) ; $ (let sum = sum + $ crate :: cmsg_aligned_space ! ($ restid ($ restex)) ;) * sum } } ; }
    };
}

cmsg_space!();