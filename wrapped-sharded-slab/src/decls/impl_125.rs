macro_rules! deps {
    () => {
        Pack!();
        Config!();
        Addr!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < C : cfg :: Config > Pack < C > for Addr < C > { const LEN : usize = C :: MAX_PAGES + C :: ADDR_INDEX_SHIFT ; type Prev = () ; fn as_usize (& self) -> usize { self . addr } fn from_usize (addr : usize) -> Self { debug_assert ! (addr <= Self :: BITS) ; Self { addr , _cfg : PhantomData , } } }
    };
}

impl_125!()