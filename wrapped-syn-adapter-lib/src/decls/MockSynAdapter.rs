macro_rules! deps {
    () => {
        SynAdapter!();
    };
}

macro_rules! MockSynAdapter {
    () => {
        deps!();
        # [doc = " Mock implementation of `SynAdapter` for dry-run or testing."] pub struct MockSynAdapter ;
    };
}

MockSynAdapter!()