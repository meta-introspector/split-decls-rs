// Generated macro for first_pass_operation (function)
macro_rules! Depcrate_first_passfirst_pass_operation {
() => {
// Module: crate::first_pass
// Provides: {"first_pass_operation"}
// Dependencies: {}
fn first_pass_operation < 'src , A : Into < Arg < 'src > > + 'src > (record : & mut FirstPassRecord < 'src > , first_pass_operation_type : FirstPassOperationType , self_name : & 'src str , ids : & [OperationId < 'src >] , arguments : impl IntoIterator < Item = A > , ret : & weedle :: types :: ReturnType < 'src > , attrs : & 'src Option < ExtendedAttributeList < 'src > > , is_static : bool , stability : ApiStability ,) { if util :: is_chrome_only (attrs) { return ; } let operations = match first_pass_operation_type { FirstPassOperationType :: Interface => { let x = record . interfaces . get_mut (self_name) . unwrap_or_else (| | panic ! ("not found {self_name} interface")) ; & mut x . operations } FirstPassOperationType :: Mixin => { let x = record . mixins . get_mut (self_name) . unwrap_or_else (| | panic ! ("not found {self_name} mixin")) ; & mut x . operations } FirstPassOperationType :: Namespace => { let x = record . namespaces . get_mut (self_name) . unwrap_or_else (| | panic ! ("not found {self_name} namespace")) ; & mut x . operations } } ; let args = arguments . into_iter () . map (Into :: into) . collect :: < Vec < _ > > () ; for id in ids { let op = operations . entry (* id) . or_default () ; op . is_static = is_static ; op . stability = stability ; op . signatures . push (Signature { args : args . clone () , ret : ret . clone () , attrs , }) ; } }
};
}
