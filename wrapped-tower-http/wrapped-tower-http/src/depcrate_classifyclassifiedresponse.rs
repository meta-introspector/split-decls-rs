// Generated macro for ClassifiedResponse (enum)
macro_rules! Depcrate_classifyClassifiedResponse {
() => {
// Module: crate::classify
// Provides: {"ClassifiedResponse"}
// Dependencies: {}
# [doc = " Result of doing a classification."] # [derive (Debug)] pub enum ClassifiedResponse < FailureClass , ClassifyEos > { # [doc = " The response was able to be classified immediately."] Ready (Result < () , FailureClass >) , # [doc = " We have to wait until the end of a streaming response to classify it."] RequiresEos (ClassifyEos) , }
};
}
