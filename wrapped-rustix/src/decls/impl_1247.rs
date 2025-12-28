macro_rules! deps {
    () => {
        Wait!();
        WaitPtr!();
    };
}

macro_rules! impl_1247 {
    () => {
        deps!();
        impl Wait { # [doc = " Construct a zero-initialized `Wait`."] # [inline] pub const fn new () -> Self { Self { val : 0 , uaddr : WaitPtr :: new (ptr :: null_mut ()) , flags : WaitFlags :: empty () , __reserved : 0 , } } }
    };
}

impl_1247!()