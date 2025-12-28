macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! copyfile_state_get {
    () => {
        deps!();
        # [doc = " `copyfile_state_get(state, flags, dst)`"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `state` operand must be allocated with `copyfile_state_alloc` and not"] # [doc = " yet freed with `copyfile_state_free`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/fcopyfile.3.html"] # [inline] pub unsafe fn copyfile_state_get (state : copyfile_state_t , flag : u32 , dst : * mut core :: ffi :: c_void ,) -> io :: Result < () > { backend :: fs :: syscalls :: copyfile_state_get (state , flag , dst) }
    };
}

copyfile_state_get!()