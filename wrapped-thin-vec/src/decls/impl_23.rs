macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < T > Drop for ThinVec < T > { # [inline] fn drop (& mut self) { # [cold] # [inline (never)] fn drop_non_singleton < T > (this : & mut ThinVec < T >) { unsafe { ptr :: drop_in_place (& mut this [..]) ; if this . uses_stack_allocated_buffer () { return ; } dealloc (this . ptr () as * mut u8 , layout :: < T > (this . capacity ())) } } if ! self . is_singleton () { drop_non_singleton (self) ; } } }
    };
}

impl_23!();