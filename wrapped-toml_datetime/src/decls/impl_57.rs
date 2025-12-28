macro_rules! deps {
    () => {
        SerializerError!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        # [cfg (all (not (feature = "std") , feature = "serde"))] impl serde_core :: de :: StdError for SerializerError { }
    };
}

impl_57!();