macro_rules! deps {
    () => {
        Error!();
        Table!();
        TableSerializer!();
    };
}

macro_rules! impl_372 {
    () => {
        deps!();
        impl Table { # [doc = " Convert a `T` into `toml::Table`."] # [doc = ""] # [doc = " This conversion can fail if `T`'s implementation of `Serialize` decides to"] # [doc = " fail, or if `T` contains a map with non-string keys."] pub fn try_from < T > (value : T) -> Result < Self , crate :: ser :: Error > where T : ser :: Serialize , { value . serialize (TableSerializer) } # [doc = " Interpret a `toml::Table` as an instance of type `T`."] # [doc = ""] # [doc = " This conversion can fail if the structure of the `Table` does not match the structure"] # [doc = " expected by `T`, for example if `T` is a bool which can't be mapped to a `Table`. It can"] # [doc = " also fail if the structure is correct but `T`'s implementation of `Deserialize` decides"] # [doc = " that something is wrong with the data, for example required struct fields are missing from"] # [doc = " the TOML map or some number is too big to fit in the expected primitive type."] pub fn try_into < 'de , T > (self) -> Result < T , crate :: de :: Error > where T : de :: Deserialize < 'de > , { de :: Deserialize :: deserialize (self) } }
    };
}

impl_372!();