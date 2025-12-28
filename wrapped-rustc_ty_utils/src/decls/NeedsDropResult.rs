macro_rules! NeedsDropResult {
    () => {
        type NeedsDropResult < T > = Result < T , AlwaysRequiresDrop > ;
    };
}

NeedsDropResult!()