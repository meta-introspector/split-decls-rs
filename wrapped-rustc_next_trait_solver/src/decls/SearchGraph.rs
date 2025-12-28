macro_rules! deps {
    () => {
        SearchGraphDelegate!();
    };
}

macro_rules! SearchGraph {
    () => {
        deps!();
        pub (super) type SearchGraph < D > = search_graph :: SearchGraph < SearchGraphDelegate < D > > ;
    };
}

SearchGraph!()