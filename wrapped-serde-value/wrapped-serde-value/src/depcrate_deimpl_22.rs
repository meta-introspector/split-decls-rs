// Generated macro for impl_22 (impl)
macro_rules! Depcrate_deimpl_22 {
() => {
// Module: crate::de
// Provides: {"impl_22"}
// Dependencies: {}
impl fmt :: Display for DeserializerError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { DeserializerError :: Custom (ref msg) => write ! (f , "{}" , msg) , DeserializerError :: InvalidType (ref unexp , ref exp) => { write ! (f , "Invalid type {}. Expected {}" , unexp . to_unexpected () , exp) } DeserializerError :: InvalidValue (ref unexp , ref exp) => { write ! (f , "Invalid value {}. Expected {}" , unexp . to_unexpected () , exp) } DeserializerError :: InvalidLength (len , ref exp) => { write ! (f , "Invalid length {}. Expected {}" , len , exp) } DeserializerError :: UnknownVariant (ref field , exp) => { write ! (f , "Unknown variant {}. Expected one of {}" , field , exp . join (", ")) } DeserializerError :: UnknownField (ref field , exp) => { write ! (f , "Unknown field {}. Expected one of {}" , field , exp . join (", ")) } DeserializerError :: MissingField (field) => write ! (f , "Missing field {}" , field) , DeserializerError :: DuplicateField (field) => write ! (f , "Duplicate field {}" , field) , } } }
};
}
