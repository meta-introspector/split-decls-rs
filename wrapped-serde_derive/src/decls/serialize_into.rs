macro_rules! deps {
    () => {
        Parameters!();
        Fragment!();
    };
}

macro_rules! serialize_into {
    () => {
        deps!();
        fn serialize_into (params : & Parameters , type_into : & syn :: Type) -> Fragment { let self_var = & params . self_var ; quote_block ! { _serde :: Serialize :: serialize (& _serde ::# private :: Into ::<# type_into >:: into (_serde ::# private :: Clone :: clone (# self_var)) , __serializer) } }
    };
}

serialize_into!()