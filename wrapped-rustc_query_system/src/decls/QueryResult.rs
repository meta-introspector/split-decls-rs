macro_rules! deps {
    () => {
        QueryJob!();
    };
}

macro_rules! QueryResult {
    () => {
        deps!();
        # [doc = " Indicates the state of a query for a given key in a query map."] enum QueryResult < I > { # [doc = " An already executing query. The query job can be used to await for its completion."] Started (QueryJob < I >) , # [doc = " The query panicked. Queries trying to wait on this will raise a fatal error which will"] # [doc = " silently panic."] Poisoned , }
    };
}

QueryResult!()