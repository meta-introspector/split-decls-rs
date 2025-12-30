// Generated macro for ProofTranscriptStep (enum)
macro_rules! Depcrate_transcriptProofTranscriptStep {
() => {
// Module: crate::transcript
// Provides: {"ProofTranscriptStep"}
// Dependencies: {}
# [doc = " Step of a proof transcript."] # [doc = ""] # [doc = " The proof transcript contains the solver queries and results that correspond to a checked proof."] # [doc = ""] # [doc = " The transcript uses the same variable numbering as used for solver calls."] # [derive (Debug)] pub enum ProofTranscriptStep < 'a > { WitnessVar { var : Var } , SampleVar { var : Var } , HideVar { var : Var } , ObserveInternalVar { var : Var } , AddClause { clause : & 'a [Lit] } , Unsat , Model { assignment : & 'a [Lit] } , Assume { assumptions : & 'a [Lit] } , FailedAssumptions { failed_core : & 'a [Lit] } , }
};
}
