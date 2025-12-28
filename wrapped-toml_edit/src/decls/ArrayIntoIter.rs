macro_rules! deps {
    () => {
        Item!();
        Value!();
    };
}

macro_rules! ArrayIntoIter {
    () => {
        deps!();
        # [doc = " An owned iterator type over [`Array`]'s [`Value`]s"] pub type ArrayIntoIter = Box < dyn Iterator < Item = Value > > ;
    };
}

ArrayIntoIter!()