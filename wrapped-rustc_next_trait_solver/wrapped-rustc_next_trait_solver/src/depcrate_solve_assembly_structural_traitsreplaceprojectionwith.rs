// Generated macro for ReplaceProjectionWith (struct)
macro_rules! Depcrate_solve_assembly_structural_traitsReplaceProjectionWith {
() => {
// Module: crate::solve::assembly::structural_traits
// Provides: {"ReplaceProjectionWith"}
// Dependencies: {}
struct ReplaceProjectionWith < 'a , 'b , I : Interner , D : SolverDelegate < Interner = I > > { ecx : & 'a mut EvalCtxt < 'b , D > , param_env : I :: ParamEnv , self_ty : I :: Ty , mapping : & 'a HashMap < I :: DefId , Vec < ty :: Binder < I , ty :: ProjectionPredicate < I > > > > , nested : Vec < Goal < I , I :: Predicate > > , }
};
}
