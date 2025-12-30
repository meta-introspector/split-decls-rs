// Generated macro for impl_89 (impl)
macro_rules! Depcrate_quic_vecimpl_89 {
() => {
// Module: crate::quic_vec
// Provides: {"impl_89"}
// Dependencies: {}
impl DeserializeBytes for VLBytes { # [inline (always)] fn tls_deserialize_bytes (bytes : & [u8]) -> Result < (Self , & [u8]) , Error > { let ((length , _) , remainder) = read_variable_length_bytes (bytes) ? ; if length == 0 { return Ok ((Self :: new (vec ! []) , remainder)) ; } if ! cfg ! (fuzzing) { debug_assert ! (length <= MAX_LEN as usize , "Trying to allocate {length} bytes. Only {MAX_LEN} allowed." ,) ; } if length > MAX_LEN as usize { return Err (Error :: DecodingError (format ! ("Trying to allocate {length} bytes. Only {MAX_LEN} allowed." ,))) ; } match remainder . get (.. length) . ok_or (Error :: EndOfStream) { Ok (vec) => Ok ((Self { vec : vec . to_vec () } , & remainder [length ..])) , Err (_e) => { let remaining_len = remainder . len () ; if ! cfg ! (fuzzing) { debug_assert_eq ! (remaining_len , length , "Expected to read {length} bytes but {remaining_len} were read." ,) ; } Err (Error :: DecodingError (format ! ("{remaining_len} bytes were read but {length} were expected" ,))) } } } }
};
}
