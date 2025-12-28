macro_rules! deps {
    () => {
        Local!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl fmt :: Debug for Local { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . head . with (| head | { let head = unsafe { * head } ; f . debug_struct ("Local") . field ("head" , & format_args ! ("{:#0x}" , head)) . finish () }) } }
    };
}

impl_135!();