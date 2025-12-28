macro_rules! deps {
    () => {
        CanonicalResponse!();
        NoSolution!();
        Certainty!();
    };
}

macro_rules! QueryResult {
    () => {
        deps!();
        # [doc = " The result of evaluating a canonical query."] # [doc = ""] # [doc = " FIXME: We use a different type than the existing canonical queries. This is because"] # [doc = " we need to add a `Certainty` for `overflow` and may want to restructure this code without"] # [doc = " having to worry about changes to currently used code. Once we've made progress on this"] # [doc = " solver, merge the two responses again."] pub type QueryResult < I > = Result < CanonicalResponse < I > , NoSolution > ;
    };
}

QueryResult!()