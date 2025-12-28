macro_rules! Config {
    () => {
        # [derive (Debug , Deserialize)] pub struct Config { pub rule : Vec < Rule > , }
    };
}

Config!();