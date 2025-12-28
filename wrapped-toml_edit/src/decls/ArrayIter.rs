macro_rules! deps {
    () => {
        Item!();
        Value!();
    };
}

macro_rules! ArrayIter {
    () => {
        deps!();
        # [doc = " An iterator type over [`Array`]'s [`Value`]s"] pub type ArrayIter < 'a > = Box < dyn Iterator < Item = & 'a Value > + 'a > ;
    };
}

ArrayIter!()