// Generated macro for MixinData (struct)
macro_rules! Depcrate_first_passMixinData {
() => {
// Module: crate::first_pass
// Provides: {"MixinData"}
// Dependencies: {}
# [doc = " We need to collect mixin data during the first pass, to be used later."] # [derive (Default)] pub (crate) struct MixinData < 'src > { # [doc = " Whether only partial mixins were encountered"] pub (crate) partial : bool , pub (crate) attributes : Vec < AttributeMixinData < 'src > > , pub (crate) consts : Vec < & 'src ConstMember < 'src > > , pub (crate) operations : BTreeMap < OperationId < 'src > , OperationData < 'src > > , pub (crate) definition_attributes : Option < & 'src ExtendedAttributeList < 'src > > , pub (crate) stability : ApiStability , }
};
}
