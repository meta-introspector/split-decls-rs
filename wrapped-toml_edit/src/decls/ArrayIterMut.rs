macro_rules! deps {
    () => {
        Value!();
        Item!();
    };
}

macro_rules! ArrayIterMut {
    () => {
        deps!();
        # [doc = " An iterator type over [`Array`]'s [`Value`]s"] pub type ArrayIterMut < 'a > = Box < dyn Iterator < Item = & 'a mut Value > + 'a > ;
    };
}

ArrayIterMut!()