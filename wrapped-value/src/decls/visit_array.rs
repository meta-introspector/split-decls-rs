macro_rules! deps {
    () => {
        Value!();
        ConstValue!();
        DeserializerError!();
        SeqDeserializer!();
    };
}

macro_rules! visit_array {
    () => {
        deps!();
        fn visit_array < 'de , V > (array : Vec < ConstValue > , visitor : V) -> Result < V :: Value , DeserializerError > where V : Visitor < 'de > , { let len = array . len () ; let mut deserializer = SeqDeserializer :: new (array) ; let seq = visitor . visit_seq (& mut deserializer) ? ; let remaining = deserializer . iter . len () ; if remaining == 0 { Ok (seq) } else { Err (DeserializerError :: invalid_length (len , & "fewer elements in array" ,)) } }
    };
}

visit_array!();