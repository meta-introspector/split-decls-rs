macro_rules! deps {
    () => {
        TomlWrite!();
        WriteTomlValue!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < V : WriteTomlValue > WriteTomlValue for [V] { fn write_toml_value < W : TomlWrite + ? Sized > (& self , writer : & mut W) -> core :: fmt :: Result { writer . open_array () ? ; let mut iter = self . iter () ; if let Some (v) = iter . next () { writer . value (v) ? ; } for v in iter { writer . val_sep () ? ; writer . space () ? ; writer . value (v) ? ; } writer . close_array () ? ; Ok (()) } }
    };
}

impl_66!();