macro_rules! deps {
    () => {
        DateTime!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl fmt :: Display for DateTime { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:04}-{:02}-{:02} {:02}:{:02}:{:02}" , self . year () , self . month () , self . day () , self . hour () , self . minute () , self . second ()) } }
    };
}

impl_199!()