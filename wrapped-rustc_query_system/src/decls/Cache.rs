macro_rules! deps {
    () => {
        Value!();
        WithDepNode!();
    };
}

macro_rules! Cache {
    () => {
        deps!();
        pub struct Cache < Key , Value > { hashmap : Lock < FxHashMap < Key , WithDepNode < Value > > > , }
    };
}

Cache!();