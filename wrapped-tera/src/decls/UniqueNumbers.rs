macro_rules! deps {
    () => {
        Unique!();
    };
}

macro_rules! UniqueNumbers {
    () => {
        deps!();
        type UniqueNumbers = Unique < i64 > ;
    };
}

UniqueNumbers!()