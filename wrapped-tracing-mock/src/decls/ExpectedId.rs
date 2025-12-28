macro_rules! deps {
    () => {
        NewSpan!();
        MockSubscriber!();
        ExpectedSpan!();
    };
}

macro_rules! ExpectedId {
    () => {
        deps!();
        # [doc = " A mock span ID."] # [doc = ""] # [doc = " This ID makes it possible to link together calls to different"] # [doc = " [`MockSubscriber`] span methods that take an [`ExpectedSpan`] in"] # [doc = " addition to those that take a [`NewSpan`]."] # [doc = ""] # [doc = " Use [`expect::id`] to construct a new, unset `ExpectedId`."] # [doc = ""] # [doc = " For more details on how to use this struct, see the documentation"] # [doc = " on [`ExpectedSpan::with_id`]."] # [doc = ""] # [doc = " [`expect::id`]: fn@crate::expect::id"] # [doc = " [`MockSubscriber`]: struct@crate::subscriber::MockSubscriber"] # [derive (Clone , Default)] pub struct ExpectedId { inner : Arc < AtomicU64 > , }
    };
}

ExpectedId!()