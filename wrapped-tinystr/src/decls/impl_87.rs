macro_rules! deps {
    () => {
        TinyAsciiStr!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < 'a , const N : usize > ZeroMapKV < 'a > for TinyAsciiStr < N > { type Container = ZeroVec < 'a , TinyAsciiStr < N > > ; type Slice = ZeroSlice < TinyAsciiStr < N > > ; type GetType = TinyAsciiStr < N > ; type OwnedType = TinyAsciiStr < N > ; }
    };
}

impl_87!();