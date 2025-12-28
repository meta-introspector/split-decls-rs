macro_rules! deps {
    () => {
        TomlIntegerFormat!();
    };
}

macro_rules! TomlInteger {
    () => {
        deps!();
        # [doc = " Helper struct for formatting TOML integers."] # [doc = ""] # [doc = " This may be constructed by calling [`TomlIntegerFormat::format()`]."] # [derive (Copy , Clone , Debug)] pub struct TomlInteger < N > { value : N , format : TomlIntegerFormat , }
    };
}

TomlInteger!()