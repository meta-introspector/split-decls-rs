// Generated macro for WorkerFn (struct)
macro_rules! Depcrate_worker_fnWorkerFn {
() => {
// Module: crate::worker_fn
// Provides: {"WorkerFn"}
// Dependencies: {}
# [derive (Clone)] pub struct WorkerFn < F > where F : WorkerFnType + 'static , { pub recv_type : F :: RecvType , pub output_type : F :: OutputType , pub generics : Generics , pub vis : Visibility , pub attrs : Vec < Attribute > , pub name : Ident , pub worker_name : Option < Ident > , pub is_async : bool , pub func : ItemFn , }
};
}
