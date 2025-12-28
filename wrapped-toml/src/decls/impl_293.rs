macro_rules! deps {
    () => {
        WalkValue!();
        SerializationStrategy!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        impl < T > From < & T > for SerializationStrategy where T : serde_core :: ser :: Serialize + ? Sized , { fn from (value : & T) -> Self { value . serialize (WalkValue) . unwrap_err () } }
    };
}

impl_293!()