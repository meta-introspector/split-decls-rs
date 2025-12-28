macro_rules! deps {
    () => {
        PollSemaphore!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl fmt :: Debug for PollSemaphore { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("PollSemaphore") . field ("semaphore" , & self . semaphore) . finish () } }
    };
}

impl_74!();