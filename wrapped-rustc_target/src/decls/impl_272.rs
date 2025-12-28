macro_rules! deps {
    () => {
        ArgAttributes!();
        ArgExtension!();
    };
}

macro_rules! impl_272 {
    () => {
        deps!();
        impl ArgAttributes { pub fn new () -> Self { ArgAttributes { regular : ArgAttribute :: default () , arg_ext : ArgExtension :: None , pointee_size : Size :: ZERO , pointee_align : None , } } pub fn ext (& mut self , ext : ArgExtension) -> & mut Self { assert ! (self . arg_ext == ArgExtension :: None || self . arg_ext == ext , "cannot set {:?} when {:?} is already set" , ext , self . arg_ext) ; self . arg_ext = ext ; self } pub fn set (& mut self , attr : ArgAttribute) -> & mut Self { self . regular |= attr ; self } pub fn contains (& self , attr : ArgAttribute) -> bool { self . regular . contains (attr) } # [doc = " Checks if these two `ArgAttributes` are equal enough to be considered \"the same for all"] # [doc = " function call ABIs\"."] pub fn eq_abi (& self , other : & Self) -> bool { if self . regular . contains (ArgAttribute :: InReg) != other . regular . contains (ArgAttribute :: InReg) { return false ; } if self . arg_ext != other . arg_ext { return false ; } true } }
    };
}

impl_272!()