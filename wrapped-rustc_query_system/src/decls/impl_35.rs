macro_rules! deps {
    () => {
        EdgesVec!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl EdgesVec { pub (crate) const INLINE_CAPACITY : usize = 8 ; # [inline] pub (crate) fn new () -> Self { Self :: default () } # [inline] pub (crate) fn push (& mut self , edge : DepNodeIndex) { self . max = self . max . max (edge . as_u32 ()) ; self . edges . push (edge) ; } # [inline] pub (crate) fn max_index (& self) -> u32 { self . max } }
    };
}

impl_35!()