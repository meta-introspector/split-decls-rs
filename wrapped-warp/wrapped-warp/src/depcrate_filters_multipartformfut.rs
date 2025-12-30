// Generated macro for FormFut (type)
macro_rules! Depcrate_filters_multipartFormFut {
() => {
// Module: crate::filters::multipart
// Provides: {"FormFut"}
// Dependencies: {}
type FormFut = Pin < Box < dyn Future < Output = Result < (FormData ,) , Rejection > > + Send > > ;
};
}
