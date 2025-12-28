macro_rules! __unsafe {
    () => {
        # [doc = " A no-op `unsafe fn` for use in macro expansions."] # [doc = ""] # [doc = " Calling this function in a macro expansion ensures that the macro's caller"] # [doc = " must wrap the call in `unsafe { ... }`."] pub (crate) const unsafe fn __unsafe () { }
    };
}

__unsafe!()