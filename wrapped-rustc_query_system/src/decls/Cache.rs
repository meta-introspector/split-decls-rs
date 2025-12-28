macro_rules! deps {
    () => {
        WithDepNode!();
        Value!();
    };
}

macro_rules! Cache {
    () => {
        deps!();
        pub struct Cache < Key , Value > { hashmap : Lock < FxHashMap < Key , WithDepNode < Value > > > , }
    };
}

Cache!()