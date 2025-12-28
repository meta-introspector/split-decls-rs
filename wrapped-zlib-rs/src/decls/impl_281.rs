macro_rules! deps {
    () => {
        DeflateFlush!();
    };
}

macro_rules! impl_281 {
    () => {
        deps!();
        impl TryFrom < i32 > for DeflateFlush { type Error = () ; fn try_from (value : i32) -> Result < Self , Self :: Error > { match value { 0 => Ok (Self :: NoFlush) , 1 => Ok (Self :: PartialFlush) , 2 => Ok (Self :: SyncFlush) , 3 => Ok (Self :: FullFlush) , 4 => Ok (Self :: Finish) , 5 => Ok (Self :: Block) , _ => Err (()) , } } }
    };
}

impl_281!()