// Generated macro for impl_3 (impl)
macro_rules! Depcrateimpl_3 {
() => {
// Module: crate
// Provides: {"impl_3"}
// Dependencies: {}
impl < T , E > std :: future :: Future for PollN < T , E > where T : Unpin , E : Unpin , { type Output = Result < T , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context) -> Poll < Self :: Output > { let this = self . get_mut () ; this . polls += 1 ; if this . polls == this . finish_at { let value = this . and_return . take () . expect ("polled after ready") ; Poll :: Ready (value) } else { cx . waker () . wake_by_ref () ; Poll :: Pending } } }
};
}
