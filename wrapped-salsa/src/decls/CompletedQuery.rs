macro_rules! deps {
    () => {
        QueryRevisions!();
        Id!();
        Identity!();
    };
}

macro_rules! CompletedQuery {
    () => {
        deps!();
        # [doc = " The state of a completed query."] pub (crate) struct CompletedQuery { # [doc = " Inputs and outputs accumulated during query execution."] pub (crate) revisions : QueryRevisions , # [doc = " The keys of any tracked structs that were created in a previous execution of the"] # [doc = " query but not the current one, and should be marked as stale."] pub (crate) stale_tracked_structs : Vec < (Identity , Id) > , }
    };
}

CompletedQuery!()