// Generated macro for impl_385 (impl)
macro_rules! Depcrate_filters_fsimpl_385 {
() => {
// Module: crate::filters::fs
// Provides: {"impl_385"}
// Dependencies: {}
impl Conditionals { fn check (self , last_modified : Option < LastModified >) -> Cond { if let Some (since) = self . if_unmodified_since { let precondition = last_modified . map (| time | since . precondition_passes (time . into ())) . unwrap_or (false) ; tracing :: trace ! ("if-unmodified-since? {:?} vs {:?} = {}" , since , last_modified , precondition) ; if ! precondition { let mut res = Response :: new (Body :: empty ()) ; * res . status_mut () = StatusCode :: PRECONDITION_FAILED ; return Cond :: NoBody (res) ; } } if let Some (since) = self . if_modified_since { tracing :: trace ! ("if-modified-since? header = {:?}, file = {:?}" , since , last_modified) ; let unmodified = last_modified . map (| time | ! since . is_modified (time . into ())) . unwrap_or (false) ; if unmodified { let mut res = Response :: new (Body :: empty ()) ; * res . status_mut () = StatusCode :: NOT_MODIFIED ; return Cond :: NoBody (res) ; } } if let Some (if_range) = self . if_range { tracing :: trace ! ("if-range? {:?} vs {:?}" , if_range , last_modified) ; let can_range = ! if_range . is_modified (None , last_modified . as_ref ()) ; if ! can_range { return Cond :: WithBody (None) ; } } Cond :: WithBody (self . range) } }
};
}
