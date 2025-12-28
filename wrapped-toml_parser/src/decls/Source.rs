macro_rules! Source {
    () => {
        # [doc = " Data encoded as TOML"] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub struct Source < 'i > { input : & 'i str , }
    };
}

Source!();