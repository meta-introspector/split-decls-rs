macro_rules! deps {
    () => {
        Keys!();
    };
}

macro_rules! KeysImpl {
    () => {
        deps!();
        # [cfg (feature = "preserve_order")] type KeysImpl < 'a , K , V > = indexmap :: map :: Keys < 'a , K , V > ;
    };
}

KeysImpl!()