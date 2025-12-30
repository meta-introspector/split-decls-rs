// Generated macro for impl_105 (impl)
macro_rules! Depcrate_timestampimpl_105 {
() => {
// Module: crate::timestamp
// Provides: {"impl_105"}
// Dependencies: {}
impl < T : ClockSequence + ? Sized > ClockSequence for & T { type Output = T :: Output ; fn generate_sequence (& self , seconds : u64 , subsec_nanos : u32) -> Self :: Output { (* * self) . generate_sequence (seconds , subsec_nanos) } fn generate_timestamp_sequence (& self , seconds : u64 , subsec_nanos : u32 ,) -> (Self :: Output , u64 , u32) { (* * self) . generate_timestamp_sequence (seconds , subsec_nanos) } fn usable_bits (& self) -> usize where Self :: Output : Sized , { (* * self) . usable_bits () } }
};
}
