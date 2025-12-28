macro_rules! deps {
    () => {
        NanosleepRelativeResult!();
        Result!();
    };
}

macro_rules! impl_1223 {
    () => {
        deps!();
        impl fmt :: Debug for NanosleepRelativeResult { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Ok => f . write_str ("Ok") , Self :: Interrupted (remaining) => write ! (f , "Interrupted(Timespec {{ tv_sec: {:?}, tv_nsec: {:?} }})" , remaining . tv_sec , remaining . tv_nsec) , Self :: Err (err) => write ! (f , "Err({:?})" , err) , } } }
    };
}

impl_1223!();