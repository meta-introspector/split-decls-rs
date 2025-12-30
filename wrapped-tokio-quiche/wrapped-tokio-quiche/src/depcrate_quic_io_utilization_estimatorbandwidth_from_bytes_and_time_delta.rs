// Generated macro for bandwidth_from_bytes_and_time_delta (function)
macro_rules! Depcrate_quic_io_utilization_estimatorbandwidth_from_bytes_and_time_delta {
() => {
// Module: crate::quic::io::utilization_estimator
// Provides: {"bandwidth_from_bytes_and_time_delta"}
// Dependencies: {}
# [doc = " Bandwidth in bits per second from bytes and time period"] fn bandwidth_from_bytes_and_time_delta (bytes : u64 , time_delta : Duration) -> u64 { if bytes == 0 { return 0 ; } let mut nanos = time_delta . as_nanos () ; if nanos == 0 { nanos = 1 ; } let num_nano_bits = 8 * bytes as u128 * 1_000_000_000 ; if num_nano_bits < nanos { return 1 ; } (num_nano_bits / nanos) as u64 }
};
}
