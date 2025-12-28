macro_rules! impl_19 {
    () => {
        # [cfg (feature = "alloc")] impl < T > private :: Sealed < T > for & mut Vec < T > { type Output = usize ; # [inline] fn parts_mut (& mut self) -> (* mut T , usize) { (self . as_mut_ptr () , self . len ()) } # [inline] unsafe fn assume_init (self , len : usize) -> Self :: Output { len } }
    };
}

impl_19!()