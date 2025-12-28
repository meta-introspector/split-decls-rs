macro_rules! WrappingConfig {
    () => {
        # [derive (Debug , Default , Serialize , Deserialize , Clone)] pub struct WrappingConfig { # [serde (default)] pub crates : Vec < String > , }
    };
}

WrappingConfig!();