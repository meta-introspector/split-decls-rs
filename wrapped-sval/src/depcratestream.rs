// Generated macro for stream (function)
macro_rules! Depcratestream {
() => {
// Module: crate
// Provides: {"stream"}
// Dependencies: {}
# [doc = "\nStream a value through a stream.\n"] pub fn stream < 'sval > (stream : & mut (impl Stream < 'sval > + ? Sized) , value : & 'sval (impl Value + ? Sized) ,) -> Result { stream . value (value) }
};
}
