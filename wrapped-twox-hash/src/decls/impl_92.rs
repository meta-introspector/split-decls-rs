macro_rules! deps {
    () => {
        SliceBackport!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < T > SliceBackport < T > for [T] { fn bp_as_chunks < const N : usize > (& self) -> (& [[T ; N]] , & [T]) { assert_ne ! (N , 0) ; let len = self . len () / N ; let (head , tail) = unsafe { self . split_at_unchecked (len * N) } ; let head = unsafe { slice :: from_raw_parts (head . as_ptr () . cast () , len) } ; (head , tail) } fn bp_as_chunks_mut < const N : usize > (& mut self) -> (& mut [[T ; N]] , & mut [T]) { assert_ne ! (N , 0) ; let len = self . len () / N ; let (head , tail) = unsafe { self . split_at_mut_unchecked (len * N) } ; let head = unsafe { slice :: from_raw_parts_mut (head . as_mut_ptr () . cast () , len) } ; (head , tail) } fn bp_as_rchunks < const N : usize > (& self) -> (& [T] , & [[T ; N]]) { assert_ne ! (N , 0) ; let len = self . len () / N ; let (head , tail) = unsafe { self . split_at_unchecked (self . len () - len * N) } ; let tail = unsafe { slice :: from_raw_parts (tail . as_ptr () . cast () , len) } ; (head , tail) } }
    };
}

impl_92!();