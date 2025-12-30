// Generated macro for NamespaceData (struct)
macro_rules! Depcrate_first_passNamespaceData {
() => {
// Module: crate::first_pass
// Provides: {"NamespaceData"}
// Dependencies: {}
# [doc = " We need to collect namespace data during the first pass, to be used later."] # [derive (Default)] pub (crate) struct NamespaceData < 'src > { pub (crate) operations : BTreeMap < OperationId < 'src > , OperationData < 'src > > , pub (crate) consts : Vec < ConstNamespaceData < 'src > > , pub (crate) stability : ApiStability , }
};
}
