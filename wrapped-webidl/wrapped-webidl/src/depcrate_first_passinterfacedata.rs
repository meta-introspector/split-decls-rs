// Generated macro for InterfaceData (struct)
macro_rules! Depcrate_first_passInterfaceData {
() => {
// Module: crate::first_pass
// Provides: {"InterfaceData"}
// Dependencies: {}
# [doc = " We need to collect interface data during the first pass, to be used later."] # [derive (Default)] pub (crate) struct InterfaceData < 'src > { # [doc = " Whether only partial interfaces were encountered"] pub (crate) partial : bool , pub (crate) has_interface : bool , pub (crate) deprecated : Option < Option < String > > , pub (crate) attributes : Vec < AttributeInterfaceData < 'src > > , pub (crate) consts : Vec < ConstData < 'src > > , pub (crate) operations : BTreeMap < OperationId < 'src > , OperationData < 'src > > , pub (crate) superclass : Option < & 'src str > , pub (crate) definition_attributes : Option < & 'src ExtendedAttributeList < 'src > > , pub (crate) stability : ApiStability , }
};
}
