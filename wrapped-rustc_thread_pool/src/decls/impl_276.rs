macro_rules! deps {
    () => {
        ThreadPool!();
    };
}

macro_rules! impl_276 {
    () => {
        deps!();
        impl fmt :: Debug for ThreadPool { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("ThreadPool") . field ("num_threads" , & self . current_num_threads ()) . field ("id" , & self . registry . id ()) . finish () } }
    };
}

impl_276!();