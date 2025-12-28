macro_rules! deps {
    () => {
        FromBytes!();
        IntoBytes!();
        KnownLayout!();
        Immutable!();
    };
}

macro_rules! SliceDst {
    () => {
        deps!();
        # [derive (KnownLayout , FromBytes , IntoBytes , Immutable)] # [repr (C)] # [allow (missing_debug_implementations , missing_copy_implementations)] pub struct SliceDst < T , U > { pub t : T , pub u : [U] , }
    };
}

SliceDst!()