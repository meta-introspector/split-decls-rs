macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! mremap_fixed {
    () => {
        deps!();
        # [doc = " `mremap(old_address, old_size, new_size, MREMAP_FIXED | flags)`—Resize,"] # [doc = " modify, and/or move a memory mapping to a specific address."] # [doc = ""] # [doc = " For `mremap` without moving to a specific address, see [`mremap`]."] # [doc = " [`mremap_fixed`]."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `old_address` and `new_address` must be aligned to the applicable page"] # [doc = " size, the range of memory starting at `old_address` and extending for"] # [doc = " `old_size` bytes, rounded up to the applicable page size, must be valid to"] # [doc = " mutate with `old_address`'s provenance, and the range of memory starting at"] # [doc = " `new_address` and extending for `new_size` bytes, rounded up to the"] # [doc = " applicable page size, must be valid to mutate with `new_address`'s"] # [doc = " provenance."] # [doc = ""] # [doc = " There must be no Rust references referring to either of those memory"] # [doc = " regions."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Linux]"] # [doc = ""] # [doc = " [Linux]: https://man7.org/linux/man-pages/man2/mremap.2.html"] # [cfg (any (target_os = "emscripten" , target_os = "linux"))] # [inline] # [doc (alias = "mremap")] pub unsafe fn mremap_fixed (old_address : * mut c_void , old_size : usize , new_size : usize , flags : MremapFlags , new_address : * mut c_void ,) -> io :: Result < * mut c_void > { backend :: mm :: syscalls :: mremap_fixed (old_address , old_size , new_size , flags , new_address) }
    };
}

mremap_fixed!();