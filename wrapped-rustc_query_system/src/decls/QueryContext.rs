macro_rules! deps {
    () => {
        HasDepContext!();
        QueryJobId!();
        QueryStackFrameExtra!();
        QueryInfo!();
        QueryMap!();
        QuerySideEffect!();
    };
}

macro_rules! QueryContext {
    () => {
        deps!();
        pub trait QueryContext : HasDepContext { type QueryInfo : Clone ; # [doc = " Gets a jobserver reference which is used to release then acquire"] # [doc = " a token while waiting on a query."] fn jobserver_proxy (& self) -> & Proxy ; fn next_job_id (self) -> QueryJobId ; # [doc = " Get the query information from the TLS context."] fn current_query_job (self) -> Option < QueryJobId > ; fn collect_active_jobs (self) -> Result < QueryMap < Self :: QueryInfo > , QueryMap < Self :: QueryInfo > > ; fn lift_query_info (self , info : & Self :: QueryInfo) -> QueryStackFrameExtra ; # [doc = " Load a side effect associated to the node in the previous session."] fn load_side_effect (self , prev_dep_node_index : SerializedDepNodeIndex ,) -> Option < QuerySideEffect > ; # [doc = " Register a side effect for the given node, for use in next session."] fn store_side_effect (self , dep_node_index : DepNodeIndex , side_effect : QuerySideEffect) ; # [doc = " Executes a job by changing the `ImplicitCtxt` to point to the"] # [doc = " new query job while it executes."] fn start_query < R > (self , token : QueryJobId , depth_limit : bool , compute : impl FnOnce () -> R) -> R ; fn depth_limit_error (self , job : QueryJobId) ; }
    };
}

QueryContext!();