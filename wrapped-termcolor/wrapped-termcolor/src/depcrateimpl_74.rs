// Generated macro for impl_74 (impl)
macro_rules! Depcrateimpl_74 {
() => {
// Module: crate
// Provides: {"impl_74"}
// Dependencies: {}
impl fmt :: Display for ParseColorError { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { use self :: ParseColorErrorKind :: * ; match self . kind { InvalidName => write ! (f , "unrecognized color name '{}'. Choose from: \
                 black, blue, green, red, cyan, magenta, yellow, \
                 white" , self . given) , InvalidAnsi256 => write ! (f , "unrecognized ansi256 color number, \
                 should be '[0-255]' (or a hex number), but is '{}'" , self . given) , InvalidRgb => write ! (f , "unrecognized RGB color triple, \
                 should be '[0-255],[0-255],[0-255]' (or a hex \
                 triple), but is '{}'" , self . given) , } } }
};
}
