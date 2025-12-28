macro_rules! deps {
    () => {
        ClockSequence!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl < T : ClockSequence + ? Sized > ClockSequence for & T { type Output = T :: Output ; fn generate_sequence (& self , seconds : u64 , subsec_nanos : u32) -> Self :: Output { (* * self) . generate_sequence (seconds , subsec_nanos) } fn generate_timestamp_sequence (& self , seconds : u64 , subsec_nanos : u32 ,) -> (Self :: Output , u64 , u32) { (* * self) . generate_timestamp_sequence (seconds , subsec_nanos) } fn usable_bits (& self) -> usize where Self :: Output : Sized , { (* * self) . usable_bits () } }
    };
}

impl_92!()