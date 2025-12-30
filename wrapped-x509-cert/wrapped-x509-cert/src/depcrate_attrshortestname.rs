// Generated macro for ShortestName (trait)
macro_rules! Depcrate_attrShortestName {
() => {
// Module: crate::attr
// Provides: {"ShortestName"}
// Dependencies: {}
# [doc = " Helper trait to bring shortest name by oid lookups to Database"] trait ShortestName { fn shortest_name_by_oid (& self , oid : & ObjectIdentifier) -> Option < & str > ; }
};
}
