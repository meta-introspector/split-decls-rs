macro_rules! SerAndDe {
    () => {
        type SerAndDe < T > = (Option < T > , Option < T >) ;
    };
}

SerAndDe!()