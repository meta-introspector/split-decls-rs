macro_rules! deps {
    () => {
        GetBitsError!();
    };
}

macro_rules! LiteralsSectionParseError {
    () => {
        deps!();
        # [derive (Debug)] # [non_exhaustive] pub enum LiteralsSectionParseError { IllegalLiteralSectionType { got : u8 } , GetBitsError (GetBitsError) , NotEnoughBytes { have : usize , need : u8 } , }
    };
}

LiteralsSectionParseError!();