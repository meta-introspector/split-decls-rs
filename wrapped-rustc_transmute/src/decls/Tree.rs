macro_rules! deps {
    () => {
        Def!();
    };
}

macro_rules! Tree {
    () => {
        deps!();
        type Tree = layout :: Tree < Def , ! , ! > ;
    };
}

Tree!()