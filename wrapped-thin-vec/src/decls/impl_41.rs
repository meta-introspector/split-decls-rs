macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        # [cfg (feature = "malloc_size_of")] impl < T : MallocSizeOf > MallocSizeOf for ThinVec < T > { fn size_of (& self , ops : & mut MallocSizeOfOps) -> usize { let mut n = self . shallow_size_of (ops) ; for elem in self . iter () { n += elem . size_of (ops) ; } n } }
    };
}

impl_41!();