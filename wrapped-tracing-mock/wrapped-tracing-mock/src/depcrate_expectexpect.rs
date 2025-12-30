// Generated macro for Expect (enum)
macro_rules! Depcrate_expectExpect {
() => {
// Module: crate::expect
// Provides: {"Expect"}
// Dependencies: {}
# [derive (Debug , Eq , PartialEq)] pub (crate) enum Expect { Event (ExpectedEvent) , FollowsFrom { consequence : ExpectedSpan , cause : ExpectedSpan , } , Enter (ExpectedSpan) , Exit (ExpectedSpan) , CloneSpan (ExpectedSpan) , DropSpan (ExpectedSpan) , Visit (ExpectedSpan , ExpectedFields) , NewSpan (NewSpan) , Nothing , }
};
}
