macro_rules! deps {
    () => {
        PairResult!();
    };
}

macro_rules! check_pair {
    () => {
        deps!();
        # [inline] fn check_pair (before : GraphemeCat , after : GraphemeCat) -> PairResult { use self :: PairResult :: * ; use crate :: tables :: grapheme :: GraphemeCat :: * ; match (before , after) { (GC_CR , GC_LF) => NotBreak , (GC_Control | GC_CR | GC_LF , _) => Break , (_ , GC_Control | GC_CR | GC_LF) => Break , (GC_L , GC_L | GC_V | GC_LV | GC_LVT) => NotBreak , (GC_LV | GC_V , GC_V | GC_T) => NotBreak , (GC_LVT | GC_T , GC_T) => NotBreak , (_ , GC_Extend | GC_ZWJ) => NotBreak , (_ , GC_SpacingMark) => Extended , (GC_Prepend , _) => Extended , (_ , GC_InCB_Consonant) => InCbConsonant , (GC_ZWJ , GC_Extended_Pictographic) => Emoji , (GC_Regional_Indicator , GC_Regional_Indicator) => Regional , (_ , _) => Break , } }
    };
}

check_pair!()