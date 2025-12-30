// Generated macro for serve (function)
macro_rules! Depcrate_serverserve {
() => {
// Module: crate::server
// Provides: {"serve"}
// Dependencies: {}
# [doc = " Create a `Server` with the provided `Filter`."] pub fn serve < F > (filter : F) -> Server < F , accept :: LazyTcp , run :: Standard > where F : Filter + Clone + Send + Sync + 'static , F :: Extract : Reply , F :: Error : IsReject , { Server { acceptor : accept :: LazyTcp , pipeline : false , filter , runner : run :: Standard , } }
};
}
