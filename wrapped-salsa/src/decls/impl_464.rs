macro_rules! deps {
    () => {
        QueryRevisionsExtraInner!();
    };
}

macro_rules! impl_464 {
    () => {
        deps!();
        impl QueryRevisionsExtraInner { # [cfg (feature = "salsa_unstable")] fn allocation_size (& self) -> usize { let QueryRevisionsExtraInner { # [cfg (feature = "accumulator")] accumulated , tracked_struct_ids , cycle_heads , iteration : _ , cycle_converged : _ , } = self ; # [cfg (feature = "accumulator")] let b = accumulated . allocation_size () ; # [cfg (not (feature = "accumulator"))] let b = 0 ; b + cycle_heads . allocation_size () + std :: mem :: size_of_val (tracked_struct_ids . as_slice ()) } }
    };
}

impl_464!()