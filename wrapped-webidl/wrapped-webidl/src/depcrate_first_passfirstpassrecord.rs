// Generated macro for FirstPassRecord (struct)
macro_rules! Depcrate_first_passFirstPassRecord {
() => {
// Module: crate::first_pass
// Provides: {"FirstPassRecord"}
// Dependencies: {}
# [doc = " Collection of constructs that may use partial."] # [derive (Default)] pub (crate) struct FirstPassRecord < 'src > { pub (crate) interfaces : BTreeMap < & 'src str , InterfaceData < 'src > > , pub (crate) enums : BTreeMap < & 'src str , EnumData < 'src > > , # [doc = " The mixins, mapping their name to the webidl ast node for the mixin."] pub (crate) mixins : BTreeMap < & 'src str , MixinData < 'src > > , pub (crate) typedefs : BTreeMap < & 'src str , & 'src weedle :: types :: Type < 'src > > , pub (crate) namespaces : BTreeMap < & 'src str , NamespaceData < 'src > > , pub (crate) includes : BTreeMap < & 'src str , BTreeSet < & 'src str > > , pub (crate) dictionaries : BTreeMap < & 'src str , DictionaryData < 'src > > , pub (crate) callbacks : BTreeSet < & 'src str > , pub (crate) iterators : BTreeSet < & 'src str > , pub (crate) async_iterators : BTreeSet < & 'src str > , pub (crate) callback_interfaces : BTreeMap < & 'src str , CallbackInterfaceData < 'src > > , }
};
}
