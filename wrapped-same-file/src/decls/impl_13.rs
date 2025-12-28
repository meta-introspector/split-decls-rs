macro_rules! deps {
    () => {
        Handle!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl PartialEq for Handle { fn eq (& self , _other : & Handle) -> bool { unreachable ! (ERROR_MESSAGE) ; } }
    };
}

impl_13!();