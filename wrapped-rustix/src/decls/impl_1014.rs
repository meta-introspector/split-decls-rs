macro_rules! deps {
    () => {
        FloatingPointMode!();
        Result!();
    };
}

macro_rules! impl_1014 {
    () => {
        deps!();
        impl TryFrom < u32 > for FloatingPointMode { type Error = io :: Errno ; fn try_from (value : u32) -> Result < Self , Self :: Error > { match value { PR_FP_MODE_FR => Ok (Self :: FloatingPointRegisters) , PR_FP_MODE_FRE => Ok (Self :: FloatingPointEmulation) , _ => Err (io :: Errno :: RANGE) , } } }
    };
}

impl_1014!()