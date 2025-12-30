// Generated macro for impl_17 (impl)
macro_rules! Depcrate_eventimpl_17 {
() => {
// Module: crate::event
// Provides: {"impl_17"}
// Dependencies: {}
impl fmt :: Debug for ExpectedEvent { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut s = f . debug_struct ("MockEvent") ; if let Some (ref name) = self . metadata . name { s . field ("name" , name) ; } if let Some (ref target) = self . metadata . target { s . field ("target" , target) ; } if let Some (ref level) = self . metadata . level { s . field ("level" , & format_args ! ("{:?}" , level)) ; } if let Some (ref fields) = self . fields { s . field ("fields" , fields) ; } if let Some (ref parent) = self . ancestry { s . field ("parent" , & format_args ! ("{:?}" , parent)) ; } if let Some (in_spans) = & self . in_spans { s . field ("in_spans" , in_spans) ; } s . finish () } }
};
}
