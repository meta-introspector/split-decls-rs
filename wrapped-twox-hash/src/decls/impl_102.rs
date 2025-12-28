macro_rules! deps {
    () => {
        Finalize64!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl Finalize for Finalize64 { type Output = u64 ; # [inline (always)] fn small (& self , secret : & Secret , seed : u64 , input : & [u8]) -> Self :: Output { impl_oneshot (secret , seed , input) } # [inline (always)] fn large (& self , vector : impl Vector , acc : [u64 ; 8] , last_block : & [u8] , last_stripe : & [u8 ; 64] , secret : & Secret , len : usize ,) -> Self :: Output { Algorithm (vector) . finalize_64 (acc , last_block , last_stripe , secret , len) } }
    };
}

impl_102!()