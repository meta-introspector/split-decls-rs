macro_rules! RangeOrOffset {
    () => {
        # [derive (Clone , Copy , Debug)] pub enum RangeOrOffset { Range (TextRange) , Offset (TextSize) , }
    };
}

RangeOrOffset!();