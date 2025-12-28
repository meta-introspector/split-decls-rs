macro_rules! macro_105 {
    () => {
        declare_lint ! { # [doc = " The `ffi_unwind_calls` lint detects calls to foreign functions or function pointers with"] # [doc = " `C-unwind` or other FFI-unwind ABIs."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " #![warn(ffi_unwind_calls)]"] # [doc = ""] # [doc = " unsafe extern \"C-unwind\" {"] # [doc = "     fn foo();"] # [doc = " }"] # [doc = ""] # [doc = " fn bar() {"] # [doc = "     unsafe { foo(); }"] # [doc = "     let ptr: unsafe extern \"C-unwind\" fn() = foo;"] # [doc = "     unsafe { ptr(); }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " {{produces}}"] # [doc = ""] # [doc = " ### Explanation"] # [doc = ""] # [doc = " For crates containing such calls, if they are compiled with `-C panic=unwind` then the"] # [doc = " produced library cannot be linked with crates compiled with `-C panic=abort`. For crates"] # [doc = " that desire this ability it is therefore necessary to avoid such calls."] pub FFI_UNWIND_CALLS , Allow , "call to foreign functions or function pointers with FFI-unwind ABI" }
    };
}

macro_105!()