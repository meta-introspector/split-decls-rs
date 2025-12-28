macro_rules! deps {
    () => {
        Config!();
        Array!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < T : fmt :: Debug , C : cfg :: Config > fmt :: Debug for Array < T , C > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let max = self . max . load (Acquire) ; let mut set = f . debug_map () ; for shard in & self . shards [0 ..= max] { let ptr = shard . 0 . load (Acquire) ; if let Some (shard) = ptr :: NonNull :: new (ptr) { set . entry (& format_args ! ("{:p}" , ptr) , unsafe { shard . as_ref () }) ; } else { set . entry (& format_args ! ("{:p}" , ptr) , & ()) ; } } set . finish () } }
    };
}

impl_157!()