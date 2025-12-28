macro_rules! deps {
    () => {
        FutureIncompatibilityReason!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl FutureIncompatibilityReason { pub fn edition (self) -> Option < Edition > { match self { Self :: EditionError (e) | Self :: EditionSemanticsChange (e) | Self :: EditionAndFutureReleaseError (e) | Self :: EditionAndFutureReleaseSemanticsChange (e) => Some (e) , FutureIncompatibilityReason :: FutureReleaseError | FutureIncompatibilityReason :: FutureReleaseSemanticsChange | FutureIncompatibilityReason :: Custom (_) => None , } } }
    };
}

impl_14!()