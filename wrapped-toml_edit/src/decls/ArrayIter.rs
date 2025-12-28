macro_rules! deps {
    () => {
        Value!();
        Item!();
    };
}

macro_rules! ArrayIter {
    () => {
        deps!();
        # [doc = " An iterator type over [`Array`]'s [`Value`]s"] pub type ArrayIter < 'a > = Box < dyn Iterator < Item = & 'a Value > + 'a > ;
    };
}

ArrayIter!();