// Generated macro for FindParamInClause (struct)
macro_rules! Depcrate_solve_assemblyFindParamInClause {
() => {
// Module: crate::solve::assembly
// Provides: {"FindParamInClause"}
// Dependencies: {}
struct FindParamInClause < 'a , 'b , D : SolverDelegate < Interner = I > , I : Interner > { ecx : & 'a mut EvalCtxt < 'b , D > , param_env : I :: ParamEnv , universes : Vec < Option < ty :: UniverseIndex > > , }
};
}
