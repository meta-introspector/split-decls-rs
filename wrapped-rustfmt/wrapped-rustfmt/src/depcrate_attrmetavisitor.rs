// Generated macro for MetaVisitor (trait)
macro_rules! Depcrate_attrMetaVisitor {
() => {
// Module: crate::attr
// Provides: {"MetaVisitor"}
// Dependencies: {}
pub (crate) trait MetaVisitor < 'ast > { fn visit_meta_item (& mut self , meta_item : & 'ast ast :: MetaItem) { match meta_item . kind { ast :: MetaItemKind :: Word => self . visit_meta_word (meta_item) , ast :: MetaItemKind :: List (ref list) => self . visit_meta_list (meta_item , list) , ast :: MetaItemKind :: NameValue (ref lit) => self . visit_meta_name_value (meta_item , lit) , } } fn visit_meta_list (& mut self , _meta_item : & 'ast ast :: MetaItem , list : & 'ast [ast :: MetaItemInner] ,) { for nm in list { self . visit_meta_item_inner (nm) ; } } fn visit_meta_word (& mut self , _meta_item : & 'ast ast :: MetaItem) { } fn visit_meta_name_value (& mut self , _meta_item : & 'ast ast :: MetaItem , _lit : & 'ast ast :: MetaItemLit ,) { } fn visit_meta_item_inner (& mut self , nm : & 'ast ast :: MetaItemInner) { match nm { ast :: MetaItemInner :: MetaItem (ref meta_item) => self . visit_meta_item (meta_item) , ast :: MetaItemInner :: Lit (ref lit) => self . visit_meta_item_lit (lit) , } } fn visit_meta_item_lit (& mut self , _lit : & 'ast ast :: MetaItemLit) { } }
};
}
