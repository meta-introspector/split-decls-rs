macro_rules! deps {
    () => {
        Local!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl Local { pub (crate) fn new () -> Self { Self { head : UnsafeCell :: new (0) , } } # [inline (always)] fn head (& self) -> usize { self . head . with (| head | unsafe { * head }) } # [inline (always)] fn set_head (& self , new_head : usize) { self . head . with_mut (| head | unsafe { * head = new_head ; }) } }
    };
}

impl_130!();