macro_rules! deps {
    () => {
        WorkerLocal!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl < T : fmt :: Debug > fmt :: Debug for WorkerLocal < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("WorkerLocal") . field ("registry" , & self . registry . id ()) . finish () } }
    };
}

impl_293!()