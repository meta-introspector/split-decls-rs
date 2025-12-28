macro_rules! deps {
    () => {
        Datetime!();
    };
}

macro_rules! VisitMap {
    () => {
        deps!();
        # [doc = " Integrate [`Datetime`][crate::Datetime] into an untagged deserialize"] # [cfg (feature = "alloc")] pub enum VisitMap < 'de > { # [doc = " The map was deserialized as a [Datetime][crate::Datetime] value"] Datetime (crate :: Datetime) , # [doc = " The map is of an unknown format and needs further deserialization"] Key (alloc :: borrow :: Cow < 'de , str >) , }
    };
}

VisitMap!();