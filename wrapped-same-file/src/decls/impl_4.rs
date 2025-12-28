macro_rules! deps {
    () => {
        Handle!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl PartialEq for Handle { fn eq (& self , other : & Handle) -> bool { (self . dev , self . ino) == (other . dev , other . ino) } }
    };
}

impl_4!()