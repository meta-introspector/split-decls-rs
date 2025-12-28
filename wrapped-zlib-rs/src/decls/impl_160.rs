macro_rules! deps {
    () => {
        Strategy!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl TryFrom < i32 > for Strategy { type Error = () ; fn try_from (value : i32) -> Result < Self , Self :: Error > { match value { 0 => Ok (Strategy :: Default) , 1 => Ok (Strategy :: Filtered) , 2 => Ok (Strategy :: HuffmanOnly) , 3 => Ok (Strategy :: Rle) , 4 => Ok (Strategy :: Fixed) , _ => Err (()) , } } }
    };
}

impl_160!();