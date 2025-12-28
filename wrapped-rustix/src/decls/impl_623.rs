macro_rules! deps {
    () => {
        SocketAddrAny!();
        AddressFamily!();
        SocketAddrLen!();
    };
}

macro_rules! impl_623 {
    () => {
        deps!();
        impl SocketAddrAny { # [doc = " Creates a socket address from `storage`, which is initialized for `len`"] # [doc = " bytes."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " if `len` is smaller than the sockaddr header or larger than"] # [doc = " `SocketAddrStorage`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = "  - `storage` must contain a valid socket address."] # [doc = "  - `len` bytes must be initialized."] # [inline] pub const unsafe fn new (storage : MaybeUninit < SocketAddrStorage > , len : SocketAddrLen) -> Self { assert ! (len as usize >= size_of ::< read_sockaddr :: sockaddr_header > ()) ; assert ! (len as usize <= size_of ::< SocketAddrStorage > ()) ; let len = NonZeroU32 :: new_unchecked (len) ; Self { storage , len } } # [doc = " Creates a socket address from reading from `ptr`, which points at `len`"] # [doc = " initialized bytes."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " if `len` is smaller than the sockaddr header or larger than"] # [doc = " `SocketAddrStorage`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = "  - `ptr` must be a pointer to memory containing a valid socket address."] # [doc = "  - `len` bytes must be initialized."] pub unsafe fn read (ptr : * const SocketAddrStorage , len : SocketAddrLen) -> Self { assert ! (len as usize >= size_of ::< read_sockaddr :: sockaddr_header > ()) ; assert ! (len as usize <= size_of ::< SocketAddrStorage > ()) ; let mut storage = MaybeUninit :: < SocketAddrStorage > :: uninit () ; core :: ptr :: copy_nonoverlapping (ptr . cast :: < u8 > () , storage . as_mut_ptr () . cast :: < u8 > () , len as usize ,) ; let len = NonZeroU32 :: new_unchecked (len) ; Self { storage , len } } # [doc = " Gets the initialized part of the storage as bytes."] # [inline] fn bytes (& self) -> & [u8] { let len = self . len . get () as usize ; unsafe { core :: slice :: from_raw_parts (self . storage . as_ptr () . cast () , len) } } # [doc = " Gets the address family of this socket address."] # [inline] pub fn address_family (& self) -> AddressFamily { unsafe { AddressFamily :: from_raw (crate :: backend :: net :: read_sockaddr :: read_sa_family (self . storage . as_ptr () . cast () ,)) } } # [doc = " Returns a raw pointer to the sockaddr."] # [inline] pub fn as_ptr (& self) -> * const SocketAddrStorage { self . storage . as_ptr () } # [doc = " Returns a raw mutable pointer to the sockaddr."] # [inline] pub fn as_mut_ptr (& mut self) -> * mut SocketAddrStorage { self . storage . as_mut_ptr () } # [doc = " Returns the length of the encoded sockaddr."] # [inline] pub fn addr_len (& self) -> SocketAddrLen { self . len . get () } }
    };
}

impl_623!()