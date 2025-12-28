macro_rules! deps {
    () => {
        WaitPtr!();
    };
}

macro_rules! impl_1242 {
    () => {
        deps!();
        impl WaitPtr { # [doc = " Construct a new `WaitPtr` holding the given raw pointer value."] # [inline] pub const fn new (ptr : * mut c_void) -> Self { Self { ptr , # [cfg (target_pointer_width = "16")] __pad16 : 0 , # [cfg (any (target_pointer_width = "16" , target_pointer_width = "32"))] __pad32 : 0 , } } }
    };
}

impl_1242!()