macro_rules! ForGuard {
    () => {
        # [doc = " `ForGuard` indicates whether we are talking about:"] # [doc = "   1. The variable for use outside of guard expressions, or"] # [doc = "   2. The temp that holds reference to (1.), which is actually what the"] # [doc = "      guard expressions see."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] enum ForGuard { RefWithinGuard , OutsideGuard , }
    };
}

ForGuard!();