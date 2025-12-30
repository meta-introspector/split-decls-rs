// Generated macro for Processing (struct)
macro_rules! Depcrate_processingProcessing {
() => {
// Module: crate::processing
// Provides: {"Processing"}
// Dependencies: {}
# [doc = " Registry of proof and transcript processors."] # [derive (Default)] pub struct Processing < 'a > { # [doc = " Registered proof processors."] pub processors : Vec < & 'a mut dyn ProofProcessor > , # [doc = " Registered transcript processors."] pub transcript_processors : Vec < & 'a mut dyn ProofTranscriptProcessor > , # [doc = " Proof step to transcript step conversion."] transcript : transcript :: Transcript , }
};
}
