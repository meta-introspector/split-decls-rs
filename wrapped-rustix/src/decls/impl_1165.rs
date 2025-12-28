macro_rules! deps {
    () => {
        Result!();
        Uname!();
    };
}

macro_rules! impl_1165 {
    () => {
        deps!();
        impl fmt :: Debug for Uname { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { # [cfg (not (linux_kernel))] { write ! (f , "{:?} {:?} {:?} {:?} {:?}" , self . sysname () , self . nodename () , self . release () , self . version () , self . machine () ,) } # [cfg (linux_kernel)] { write ! (f , "{:?} {:?} {:?} {:?} {:?} {:?}" , self . sysname () , self . nodename () , self . release () , self . version () , self . machine () , self . domainname () ,) } } }
    };
}

impl_1165!();