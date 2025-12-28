macro_rules! DelimSpan {
    () => {
        # [derive (Debug , Copy , Clone , PartialEq)] pub struct DelimSpan < S > { pub open : S , pub close : S , }
    };
}

DelimSpan!();