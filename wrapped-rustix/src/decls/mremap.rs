macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! mremap {
    () => {
        deps!();
        # [doc = " `mremap(old_address, old_size, new_size, flags)`—Resize, modify, and/or"] # [doc = " move a memory mapping."] # [doc = ""] # [doc = " For moving a mapping to a fixed address (`MREMAP_FIXED`), see"] # [doc = " [`mremap_fixed`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `old_address` must be aligned to the applicable page size, and the range of"] # [doc = " memory starting at `old_address` and extending for `old_size` bytes,"] # [doc = " rounded up to the applicable page size, must be valid to mutate with"] # [doc = " `old_address`'s provenance. If `MremapFlags::MAY_MOVE` is set in `flags`,"] # [doc = " there must be no Rust references referring to that the memory."] # [doc = ""] # [doc = " If `new_size` is less than `old_size`, than there must be no Rust"] # [doc = " references referring to the memory starting at offset `new_size` and ending"] # [doc = " at `old_size`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/mremap.2.html"] # [cfg (any (target_os = "emscripten" , target_os = "linux"))] # [inline] pub unsafe fn mremap (old_address : * mut c_void , old_size : usize , new_size : usize , flags : MremapFlags ,) -> io :: Result < * mut c_void > { backend :: mm :: syscalls :: mremap (old_address , old_size , new_size , flags) }
    };
}

mremap!();