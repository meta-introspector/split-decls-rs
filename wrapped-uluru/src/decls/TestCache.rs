macro_rules! deps {
    () => {
        LRUCache!();
    };
}

macro_rules! TestCache {
    () => {
        deps!();
        type TestCache = LRUCache < i32 , 4 > ;
    };
}

TestCache!()