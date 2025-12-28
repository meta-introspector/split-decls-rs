macro_rules! deps {
    () => {
        Coordinates!();
        ModulusSize!();
        Tag!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < Size : ModulusSize > Coordinates < '_ , Size > { # [doc = " Get the tag octet needed to encode this set of [`Coordinates`]"] pub fn tag (& self) -> Tag { match self { Coordinates :: Compact { .. } => Tag :: Compact , Coordinates :: Compressed { y_is_odd , .. } => { if * y_is_odd { Tag :: CompressedOddY } else { Tag :: CompressedEvenY } } Coordinates :: Identity => Tag :: Identity , Coordinates :: Uncompressed { .. } => Tag :: Uncompressed , } } }
    };
}

impl_24!();