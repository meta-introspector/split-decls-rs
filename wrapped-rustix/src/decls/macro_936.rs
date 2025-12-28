macro_rules! macro_936 {
    () => {
        bitflags ! { # [doc = " Zero means floating point exceptions are disabled."] # [repr (transparent)] # [derive (Copy , Clone , Eq , PartialEq , Hash , Debug)] pub struct FloatingPointExceptionMode : u32 { # [doc = " Async non-recoverable exception mode."] const NONRECOV = 1 ; # [doc = " Async recoverable exception mode."] const ASYNC = 2 ; # [doc = " Precise exception mode."] const PRECISE = 3 ; # [doc = " Use FPEXC for floating point exception enables."] const SW_ENABLE = 0x80 ; # [doc = " Floating point divide by zero."] const DIV = 0x01_0000 ; # [doc = " Floating point overflow."] const OVF = 0x02_0000 ; # [doc = " Floating point underflow."] const UND = 0x04_0000 ; # [doc = " Floating point inexact result."] const RES = 0x08_0000 ; # [doc = " Floating point invalid operation."] const INV = 0x10_0000 ; } }
    };
}

macro_936!()