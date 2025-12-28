macro_rules! deps {
    () => {
        TargetSpecJson!();
    };
}

macro_rules! json_schema {
    () => {
        deps!();
        pub fn json_schema () -> schemars :: Schema { schemars :: schema_for ! (TargetSpecJson) }
    };
}

json_schema!()