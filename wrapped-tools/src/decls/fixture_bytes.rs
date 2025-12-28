macro_rules! deps {
    () => {
        DirectoryRoot!();
    };
}

macro_rules! fixture_bytes {
    () => {
        deps!();
        # [doc = " Load the fixture from `<crate-root>/tests/fixtures/<path>` and return its data, or _panic_."] pub fn fixture_bytes (path : impl AsRef < Path >) -> Vec < u8 > { fixture_bytes_inner (path , DirectoryRoot :: IntegrationTest) }
    };
}

fixture_bytes!()