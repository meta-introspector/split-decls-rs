macro_rules! deps {
    () => {
        Radix!();
    };
}

macro_rules! TomlIntegerFormat {
    () => {
        deps!();
        # [doc = " Describes how a TOML integer should be formatted."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[cfg(feature = \"alloc\")] {"] # [doc = " # use toml_writer::ToTomlValue as _;"] # [doc = " let format = toml_writer::TomlIntegerFormat::new().as_hex_lower();"] # [doc = " let number = 10;"] # [doc = " let number = format.format(number).unwrap_or(toml_writer::TomlInteger::new(number));"] # [doc = " let number = number.to_toml_value();"] # [doc = " assert_eq!(number, \"0xa\");"] # [doc = " # }"] # [doc = " ```"] # [derive (Copy , Clone , Debug)] pub struct TomlIntegerFormat { radix : Radix , }
    };
}

TomlIntegerFormat!()