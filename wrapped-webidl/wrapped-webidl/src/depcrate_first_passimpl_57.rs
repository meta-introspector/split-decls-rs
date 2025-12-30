// Generated macro for impl_57 (impl)
macro_rules! Depcrate_first_passimpl_57 {
() => {
// Module: crate::first_pass
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'src > FirstPass < 'src , ApiStability > for weedle :: InterfaceDefinition < 'src > { fn first_pass (& 'src self , record : & mut FirstPassRecord < 'src > , stability : ApiStability ,) -> Result < () > { if util :: is_chrome_only (& self . attributes) { return Ok (()) ; } let interface_data = record . interfaces . entry (self . identifier . 0) . or_default () ; interface_data . partial = false ; interface_data . superclass = self . inheritance . map (| s | s . identifier . 0) ; interface_data . definition_attributes = self . attributes . as_ref () ; interface_data . deprecated = util :: get_rust_deprecated (& self . attributes) ; interface_data . has_interface = ! util :: is_no_interface_object (& self . attributes) ; interface_data . stability = stability ; if let Some (attrs) = & self . attributes { for attr in attrs . body . list . iter () { process_interface_attribute (record , self . identifier . 0 , attr) ; } } for member in & self . members . body { member . first_pass (record , (self . identifier . 0 , stability)) ? ; } Ok (()) } }
};
}
