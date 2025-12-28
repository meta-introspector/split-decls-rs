macro_rules! deps {
    () => {
        Array!();
        ArrayVec!();
        TryFromSliceError!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < T , A > TryFrom < & '_ [T] > for ArrayVec < A > where T : Clone + Default , A : Array < Item = T > , { type Error = TryFromSliceError ; # [inline] # [doc = " The output has a length equal to that of the slice, with the same capacity"] # [doc = " as `A`."] fn try_from (slice : & [T]) -> Result < Self , Self :: Error > { if slice . len () > A :: CAPACITY { Err (TryFromSliceError (())) } else { let mut arr = ArrayVec :: new () ; arr . set_len (slice . len ()) ; arr . as_mut_slice () . clone_from_slice (slice) ; Ok (arr) } } }
    };
}

impl_39!();