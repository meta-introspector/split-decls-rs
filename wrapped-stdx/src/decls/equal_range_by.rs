macro_rules! equal_range_by {
    () => {
        pub fn equal_range_by < T , F > (slice : & [T] , mut key : F) -> ops :: Range < usize > where F : FnMut (& T) -> Ordering , { let start = slice . partition_point (| it | key (it) == Ordering :: Less) ; let len = slice [start ..] . partition_point (| it | key (it) == Ordering :: Equal) ; start .. start + len }
    };
}

equal_range_by!()