macro_rules! deps {
    () => {
        GetDisjointMutError!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl fmt :: Display for GetDisjointMutError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let msg = match self { GetDisjointMutError :: IndexVacant => "an index is vacant" , GetDisjointMutError :: IndexOutOfBounds => "an index is out of bounds" , GetDisjointMutError :: OverlappingIndices => "there were overlapping indices" , } ; fmt :: Display :: fmt (msg , f) } }
    };
}

impl_9!()