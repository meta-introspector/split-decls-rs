macro_rules! impl_18 {
    () => {
        impl < T , const N : usize > private :: Sealed < T > for & mut [T ; N] { type Output = usize ; # [inline] fn parts_mut (& mut self) -> (* mut T , usize) { (self . as_mut_ptr () , N) } # [inline] unsafe fn assume_init (self , len : usize) -> Self :: Output { len } }
    };
}

impl_18!();