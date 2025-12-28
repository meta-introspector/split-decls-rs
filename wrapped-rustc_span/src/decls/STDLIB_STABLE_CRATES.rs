macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! STDLIB_STABLE_CRATES {
    () => {
        deps!();
        # [doc = " Symbols for crates that are part of the stable standard library: `std`, `core`, `alloc`, and"] # [doc = " `proc_macro`."] pub const STDLIB_STABLE_CRATES : & [Symbol] = & [sym :: std , sym :: core , sym :: alloc , sym :: proc_macro] ;
    };
}

STDLIB_STABLE_CRATES!()