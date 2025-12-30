// Generated macro for MakeClassifier (trait)
macro_rules! Depcrate_classifyMakeClassifier {
() => {
// Module: crate::classify
// Provides: {"MakeClassifier"}
// Dependencies: {}
# [doc = " Trait for producing response classifiers from a request."] # [doc = ""] # [doc = " This is useful when a classifier depends on data from the request. For example, this could"] # [doc = " include the URI or HTTP method."] # [doc = ""] # [doc = " This trait is generic over the [`Error` type] of the `Service`s used with the classifier."] # [doc = " This is necessary for [`ClassifyResponse::classify_error`]."] # [doc = ""] # [doc = " [`Error` type]: https://docs.rs/tower/latest/tower/trait.Service.html#associatedtype.Error"] pub trait MakeClassifier { # [doc = " The response classifier produced."] type Classifier : ClassifyResponse < FailureClass = Self :: FailureClass , ClassifyEos = Self :: ClassifyEos , > ; # [doc = " The type of failure classifications."] # [doc = ""] # [doc = " This might include additional information about the error, such as"] # [doc = " whether it was a client or server error, or whether or not it should"] # [doc = " be considered retryable."] type FailureClass ; # [doc = " The type used to classify the response end of stream (EOS)."] type ClassifyEos : ClassifyEos < FailureClass = Self :: FailureClass > ; # [doc = " Returns a response classifier for this request"] fn make_classifier < B > (& self , req : & Request < B >) -> Self :: Classifier ; }
};
}
