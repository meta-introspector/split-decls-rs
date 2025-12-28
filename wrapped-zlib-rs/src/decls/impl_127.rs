macro_rules! deps {
    () => {
        Method!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl TryFrom < i32 > for Method { type Error = () ; fn try_from (value : i32) -> Result < Self , Self :: Error > { match value { 8 => Ok (Self :: Deflated) , _ => Err (()) , } } }
    };
}

impl_127!();