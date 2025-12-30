// Generated macro for stream_computed (function)
macro_rules! Depcratestream_computed {
() => {
// Module: crate
// Provides: {"stream_computed"}
// Dependencies: {}
# [doc = "\nStream a value through a stream with an arbitrarily short lifetime.\n"] pub fn stream_computed < 'sval > (stream : & mut (impl Stream < 'sval > + ? Sized) , value : impl Value ,) -> Result { stream . value_computed (& value) }
};
}
