// Generated macro for impl_170 (impl)
macro_rules! Depcrate_serial_numberimpl_170 {
() => {
// Module: crate::serial_number
// Provides: {"impl_170"}
// Dependencies: {}
impl < 'a , P : Profile > DecodeValue < 'a > for SerialNumber < P > { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , header : Header) -> Result < Self > { let inner = Int :: decode_value (reader , header) ? ; let serial = Self { inner , _profile : PhantomData , } ; P :: check_serial_number (& serial) ? ; Ok (serial) } }
};
}
