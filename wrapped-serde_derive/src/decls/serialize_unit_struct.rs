macro_rules! deps {
    () => {
        Container!();
        Fragment!();
    };
}

macro_rules! serialize_unit_struct {
    () => {
        deps!();
        fn serialize_unit_struct (cattrs : & attr :: Container) -> Fragment { let type_name = cattrs . name () . serialize_name () ; quote_expr ! { _serde :: Serializer :: serialize_unit_struct (__serializer , # type_name) } }
    };
}

serialize_unit_struct!()