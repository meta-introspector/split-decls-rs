macro_rules! deps {
    () => {
        Finalize128!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl Finalize for Finalize128 { type Output = u128 ; # [inline] fn small (& self , secret : & Secret , seed : u64 , input : & [u8]) -> Self :: Output { impl_oneshot (secret , seed , input) } # [inline] fn large (& self , vector : impl Vector , acc : [u64 ; 8] , last_block : & [u8] , last_stripe : & [u8 ; 64] , secret : & Secret , len : usize ,) -> Self :: Output { Algorithm (vector) . finalize_128 (acc , last_block , last_stripe , secret , len) } }
    };
}

impl_121!()