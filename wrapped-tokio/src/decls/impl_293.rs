macro_rules! deps {
    () => {
        WakeList!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl Drop for WakeList { fn drop (& mut self) { let slice = ptr :: slice_from_raw_parts_mut (self . inner . as_mut_ptr () . cast :: < Waker > () , self . curr) ; unsafe { ptr :: drop_in_place (slice) } ; } }
    };
}

impl_293!()