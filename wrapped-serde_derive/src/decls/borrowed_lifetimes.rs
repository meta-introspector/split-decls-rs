macro_rules! deps {
    () => {
        Container!();
        BorrowedLifetimes!();
    };
}

macro_rules! borrowed_lifetimes {
    () => {
        deps!();
        fn borrowed_lifetimes (cont : & Container) -> BorrowedLifetimes { let mut lifetimes = BTreeSet :: new () ; for field in cont . data . all_fields () { if ! field . attrs . skip_deserializing () { lifetimes . extend (field . attrs . borrowed_lifetimes () . iter () . cloned ()) ; } } if lifetimes . iter () . any (| b | b . to_string () == "'static") { BorrowedLifetimes :: Static } else { BorrowedLifetimes :: Borrowed (lifetimes) } }
    };
}

borrowed_lifetimes!();