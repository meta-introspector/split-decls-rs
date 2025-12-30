// Generated macro for inner_unexpected (function)
macro_rules! Depcrate_parseinner_unexpected {
() => {
// Module: crate::parse
// Provides: {"inner_unexpected"}
// Dependencies: {}
fn inner_unexpected (buffer : & ParseBuffer) -> (Rc < Cell < Unexpected > > , Option < (Span , Delimiter) >) { let mut unexpected = get_unexpected (buffer) ; loop { match cell_clone (& unexpected) { Unexpected :: None => return (unexpected , None) , Unexpected :: Some (span , delimiter) => return (unexpected , Some ((span , delimiter))) , Unexpected :: Chain (next) => unexpected = next , } } }
};
}
