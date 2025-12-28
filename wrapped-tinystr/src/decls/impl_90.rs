macro_rules! deps {
    () => {
        UnvalidatedTinyAsciiStr!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < 'a , const N : usize > ZeroMapKV < 'a > for UnvalidatedTinyAsciiStr < N > { type Container = ZeroVec < 'a , UnvalidatedTinyAsciiStr < N > > ; type Slice = ZeroSlice < UnvalidatedTinyAsciiStr < N > > ; type GetType = UnvalidatedTinyAsciiStr < N > ; type OwnedType = UnvalidatedTinyAsciiStr < N > ; }
    };
}

impl_90!()