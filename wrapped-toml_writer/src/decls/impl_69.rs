macro_rules! deps {
    () => {
        TomlWrite!();
        WriteTomlKey!();
        WriteTomlValue!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < K : WriteTomlKey , V : WriteTomlValue > WriteTomlValue for alloc :: collections :: BTreeMap < K , V > { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { write_toml_inline_table (self . iter () , writer) } }
    };
}

impl_69!()