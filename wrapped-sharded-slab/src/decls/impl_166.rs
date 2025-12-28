macro_rules! deps {
    () => {
        Addr!();
        Config!();
        Pack!();
        Tid!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < C : cfg :: Config > Pack < C > for Tid < C > { const LEN : usize = C :: MAX_SHARDS . trailing_zeros () as usize + 1 ; type Prev = page :: Addr < C > ; # [inline (always)] fn as_usize (& self) -> usize { self . id } # [inline (always)] fn from_usize (id : usize) -> Self { Self { id , _not_send : PhantomData , _cfg : PhantomData , } } }
    };
}

impl_166!()