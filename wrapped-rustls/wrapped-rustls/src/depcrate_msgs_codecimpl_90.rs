// Generated macro for impl_90 (impl)
macro_rules! Depcrate_msgs_codecimpl_90 {
() => {
// Module: crate::msgs::codec
// Provides: {"impl_90"}
// Dependencies: {}
impl ListLength { pub (crate) fn read (& self , r : & mut Reader < '_ >) -> Result < usize , InvalidMessage > { Ok (match self { Self :: NonZeroU8 { empty_error } => match usize :: from (u8 :: read (r) ?) { 0 => return Err (* empty_error) , len => len , } , Self :: U16 => usize :: from (u16 :: read (r) ?) , Self :: NonZeroU16 { empty_error } => match usize :: from (u16 :: read (r) ?) { 0 => return Err (* empty_error) , len => len , } , Self :: U24 { max , error } => match usize :: from (u24 :: read (r) ?) { len if len > * max => return Err (* error) , len => len , } , Self :: NonZeroU24 { max , empty_error , too_many_error , } => match usize :: from (u24 :: read (r) ?) { 0 => return Err (* empty_error) , len if len > * max => return Err (* too_many_error) , len => len , } , }) } }
};
}
