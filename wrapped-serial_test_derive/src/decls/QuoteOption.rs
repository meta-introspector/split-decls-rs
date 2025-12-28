macro_rules! QuoteOption {
    () => {
        # [derive (Default , Debug , Clone)] struct QuoteOption < T > (Option < T >) ;
    };
}

QuoteOption!()