macro_rules! deps {
    () => {
        UTerm!();
        PrivateCmp!();
        Ord!();
    };
}

macro_rules! impl_436 {
    () => {
        deps!();
        # [doc = " Got to the end of both! Return `SoFar`"] impl < SoFar : Ord > PrivateCmp < UTerm , SoFar > for UTerm { type Output = SoFar ; # [inline] fn private_cmp (& self , _ : & UTerm , so_far : SoFar) -> Self :: Output { so_far } }
    };
}

impl_436!();