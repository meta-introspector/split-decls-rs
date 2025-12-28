macro_rules! deps {
    () => {
        Tag!();
    };
}

macro_rules! Index {
    () => {
        deps!();
        # [doc = "\nThe index of a value in its parent context.\n"] # [derive (Clone)] pub struct Index (i128 , Option < Tag >) ;
    };
}

Index!();