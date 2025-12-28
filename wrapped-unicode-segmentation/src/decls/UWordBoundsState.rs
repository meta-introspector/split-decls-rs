macro_rules! deps {
    () => {
        FormatExtendType!();
        RegionalState!();
    };
}

macro_rules! UWordBoundsState {
    () => {
        deps!();
        # [derive (Clone , Copy , PartialEq , Eq , Debug)] enum UWordBoundsState { Start , Letter , HLetter , Numeric , Katakana , ExtendNumLet , Regional (RegionalState) , FormatExtend (FormatExtendType) , Zwj , Emoji , WSegSpace , }
    };
}

UWordBoundsState!();