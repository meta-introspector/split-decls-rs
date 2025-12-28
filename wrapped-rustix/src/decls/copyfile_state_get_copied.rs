macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! copyfile_state_get_copied {
    () => {
        deps!();
        # [doc = " `copyfile_state_get(state, COPYFILE_STATE_COPIED)`"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `state` operand must be allocated with `copyfile_state_alloc` and not"] # [doc = " yet freed with `copyfile_state_free`."] # [doc = ""] # [doc = " # References"] # [doc = "  - [Apple]"] # [doc = ""] # [doc = " [Apple]: https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/fcopyfile.3.html"] # [inline] pub unsafe fn copyfile_state_get_copied (state : copyfile_state_t) -> io :: Result < u64 > { backend :: fs :: syscalls :: copyfile_state_get_copied (state) }
    };
}

copyfile_state_get_copied!()