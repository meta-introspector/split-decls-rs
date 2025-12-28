macro_rules! deps {
    () => {
        Identifier!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl Identifier { pub (crate) const fn empty () -> Self { const HEAD : NonNull < u8 > = unsafe { NonNull :: new_unchecked (! 0 as * mut u8) } ; Identifier { head : HEAD , tail : [! 0 ; TAIL_BYTES] , } } pub (crate) unsafe fn new_unchecked (string : & str) -> Self { let len = string . len () ; debug_assert ! (len <= isize :: MAX as usize) ; match len as u64 { 0 => Self :: empty () , 1 ..= 8 => { let mut bytes = [0u8 ; mem :: size_of :: < Identifier > ()] ; unsafe { ptr :: copy_nonoverlapping (string . as_ptr () , bytes . as_mut_ptr () , len) } ; unsafe { mem :: transmute :: < [u8 ; mem :: size_of :: < Identifier > ()] , Identifier > (bytes) } } 9 ..= 0xff_ffff_ffff_ffff => { let size = bytes_for_varint (unsafe { NonZeroUsize :: new_unchecked (len) }) + len ; let align = 2 ; if mem :: size_of :: < usize > () < 8 { let max_alloc = usize :: MAX / 2 - align ; assert ! (size <= max_alloc) ; } let layout = unsafe { Layout :: from_size_align_unchecked (size , align) } ; let ptr = unsafe { alloc (layout) } ; if ptr . is_null () { handle_alloc_error (layout) ; } let mut write = ptr ; let mut varint_remaining = len ; while varint_remaining > 0 { unsafe { ptr :: write (write , varint_remaining as u8 | 0x80) } ; varint_remaining >>= 7 ; write = unsafe { write . add (1) } ; } unsafe { ptr :: copy_nonoverlapping (string . as_ptr () , write , len) } ; Identifier { head : ptr_to_repr (ptr) , tail : [0 ; TAIL_BYTES] , } } 0x100_0000_0000_0000 ..= 0xffff_ffff_ffff_ffff => { unreachable ! ("please refrain from storing >64 petabytes of text in semver version") ; } } } pub (crate) fn is_empty (& self) -> bool { let empty = Self :: empty () ; let is_empty = self . head == empty . head && self . tail == empty . tail ; mem :: forget (empty) ; is_empty } fn is_inline (& self) -> bool { self . head . as_ptr () as usize >> (PTR_BYTES * 8 - 1) == 0 } fn is_empty_or_inline (& self) -> bool { self . is_empty () || self . is_inline () } pub (crate) fn as_str (& self) -> & str { if self . is_empty () { "" } else if self . is_inline () { unsafe { inline_as_str (self) } } else { unsafe { ptr_as_str (& self . head) } } } pub (crate) fn ptr_eq (& self , rhs : & Self) -> bool { self . head == rhs . head && self . tail == rhs . tail } }
    };
}

impl_34!();