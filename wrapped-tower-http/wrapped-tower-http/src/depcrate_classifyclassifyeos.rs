// Generated macro for ClassifyEos (trait)
macro_rules! Depcrate_classifyClassifyEos {
() => {
// Module: crate::classify
// Provides: {"ClassifyEos"}
// Dependencies: {}
# [doc = " Trait for classifying end of streams (EOS) as either success or failure."] pub trait ClassifyEos { # [doc = " The type of failure classifications."] type FailureClass ; # [doc = " Perform the classification from response trailers."] fn classify_eos (self , trailers : Option < & HeaderMap >) -> Result < () , Self :: FailureClass > ; # [doc = " Classify an error."] # [doc = ""] # [doc = " Errors are always errors (doh) but sometimes it might be useful to have multiple classes of"] # [doc = " errors. A retry policy might allow retrying some errors and not others."] fn classify_error < E > (self , error : & E) -> Self :: FailureClass where E : fmt :: Display + 'static ; # [doc = " Transform the failure classification using a function."] # [doc = ""] # [doc = " See [`ClassifyResponse::map_failure_class`] for more details."] fn map_failure_class < F , NewClass > (self , f : F) -> MapFailureClass < Self , F > where Self : Sized , F : FnOnce (Self :: FailureClass) -> NewClass , { MapFailureClass :: new (self , f) } }
};
}
