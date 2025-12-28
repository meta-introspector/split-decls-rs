macro_rules! MockConfig {
    () => {
        # [derive (Debug , Deserialize)] pub struct MockConfig { pub rule : Vec < Rule > , }
    };
}

MockConfig!();