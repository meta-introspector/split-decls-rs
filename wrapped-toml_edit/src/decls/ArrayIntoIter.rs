macro_rules! deps {
    () => {
        Value!();
        Item!();
    };
}

macro_rules! ArrayIntoIter {
    () => {
        deps!();
        # [doc = " An owned iterator type over [`Array`]'s [`Value`]s"] pub type ArrayIntoIter = Box < dyn Iterator < Item = Value > > ;
    };
}

ArrayIntoIter!();