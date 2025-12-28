macro_rules! deps {
    () => {
        ExpectedSpan!();
        ExpectedEvent!();
        ExpectedFields!();
        NewSpan!();
    };
}

macro_rules! Expect {
    () => {
        deps!();
        # [derive (Debug , Eq , PartialEq)] pub (crate) enum Expect { Event (ExpectedEvent) , FollowsFrom { consequence : ExpectedSpan , cause : ExpectedSpan , } , Enter (ExpectedSpan) , Exit (ExpectedSpan) , CloneSpan (ExpectedSpan) , DropSpan (ExpectedSpan) , Visit (ExpectedSpan , ExpectedFields) , NewSpan (NewSpan) , Nothing , }
    };
}

Expect!();