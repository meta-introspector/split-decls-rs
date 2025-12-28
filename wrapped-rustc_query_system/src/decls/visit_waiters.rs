macro_rules! deps {
    () => {
        Waiter!();
        QueryMap!();
        QueryJobId!();
    };
}

macro_rules! visit_waiters {
    () => {
        deps!();
        # [doc = " Visits all the non-resumable and resumable waiters of a query."] # [doc = " Only waiters in a query are visited."] # [doc = " `visit` is called for every waiter and is passed a query waiting on `query_ref`"] # [doc = " and a span indicating the reason the query waited on `query_ref`."] # [doc = " If `visit` returns Some, this function returns."] # [doc = " For visits of non-resumable waiters it returns the return value of `visit`."] # [doc = " For visits of resumable waiters it returns Some(Some(Waiter)) which has the"] # [doc = " required information to resume the waiter."] # [doc = " If all `visit` calls returns None, this function also returns None."] fn visit_waiters < I , F > (query_map : & QueryMap < I > , query : QueryJobId , mut visit : F ,) -> Option < Option < Waiter > > where F : FnMut (Span , QueryJobId) -> Option < Option < Waiter > > , { if let Some (parent) = query . parent (query_map) && let Some (cycle) = visit (query . span (query_map) , parent) { return Some (cycle) ; } if let Some (latch) = query . latch (query_map) { for (i , waiter) in latch . info . lock () . waiters . iter () . enumerate () { if let Some (waiter_query) = waiter . query { if visit (waiter . span , waiter_query) . is_some () { return Some (Some ((query , i))) ; } } } } None }
    };
}

visit_waiters!()