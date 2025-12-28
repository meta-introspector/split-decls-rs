macro_rules! deps {
    () => {
        Aligned!();
        Tag!();
    };
}

macro_rules! TaggedRef {
    () => {
        deps!();
        # [doc = " A covariant [`Copy`] tagged borrow. This is essentially `{ pointer: &'a P, tag: T }` packed"] # [doc = " in a single reference."] pub struct TaggedRef < 'a , Pointee : Aligned + ? Sized , T : Tag > { # [doc = " This is semantically a pair of `pointer: &'a P` and `tag: T` fields,"] # [doc = " however we pack them in a single pointer, to save space."] # [doc = ""] # [doc = " We pack the tag into the **most**-significant bits of the pointer to"] # [doc = " ease retrieval of the value. A left shift is a multiplication and"] # [doc = " those are embeddable in instruction encoding, for example:"] # [doc = ""] # [doc = " ```asm"] # [doc = " // (<https://godbolt.org/z/jqcYPWEr3>)"] # [doc = " example::shift_read3:"] # [doc = "     mov     eax, dword ptr [8*rdi]"] # [doc = "     ret"] # [doc = ""] # [doc = " example::mask_read3:"] # [doc = "     and     rdi, -8"] # [doc = "     mov     eax, dword ptr [rdi]"] # [doc = "     ret"] # [doc = " ```"] # [doc = ""] # [doc = " This is ASM outputted by rustc for reads of values behind tagged"] # [doc = " pointers for different approaches of tagging:"] # [doc = " - `shift_read3` uses `<< 3` (the tag is in the most-significant bits)"] # [doc = " - `mask_read3` uses `& !0b111` (the tag is in the least-significant bits)"] # [doc = ""] # [doc = " The shift approach thus produces less instructions and is likely faster"] # [doc = " (see <https://godbolt.org/z/Y913sMdWb>)."] # [doc = ""] # [doc = " Encoding diagram:"] # [doc = " ```text"] # [doc = " [ packed.addr                     ]"] # [doc = " [ tag ] [ pointer.addr >> T::BITS ] <-- usize::BITS - T::BITS bits"] # [doc = "    ^"] # [doc = "    |"] # [doc = " T::BITS bits"] # [doc = " ```"] # [doc = ""] # [doc = " The tag can be retrieved by `packed.addr() >> T::BITS` and the pointer"] # [doc = " can be retrieved by `packed.map_addr(|addr| addr << T::BITS)`."] packed : NonNull < Pointee > , tag_pointer_ghost : PhantomData < (& 'a Pointee , T) > , }
    };
}

TaggedRef!();