// Generated macro for impl_506 (impl)
macro_rules! Depcrate_itemimpl_506 {
() => {
// Module: crate::item
// Provides: {"impl_506"}
// Dependencies: {}
impl Item { # [cfg (feature = "parsing")] pub (crate) fn replace_attrs (& mut self , new : Vec < Attribute >) -> Vec < Attribute > { match self { Item :: Const (ItemConst { attrs , .. }) | Item :: Enum (ItemEnum { attrs , .. }) | Item :: ExternCrate (ItemExternCrate { attrs , .. }) | Item :: Fn (ItemFn { attrs , .. }) | Item :: ForeignMod (ItemForeignMod { attrs , .. }) | Item :: Impl (ItemImpl { attrs , .. }) | Item :: Macro (ItemMacro { attrs , .. }) | Item :: Mod (ItemMod { attrs , .. }) | Item :: Static (ItemStatic { attrs , .. }) | Item :: Struct (ItemStruct { attrs , .. }) | Item :: Trait (ItemTrait { attrs , .. }) | Item :: TraitAlias (ItemTraitAlias { attrs , .. }) | Item :: Type (ItemType { attrs , .. }) | Item :: Union (ItemUnion { attrs , .. }) | Item :: Use (ItemUse { attrs , .. }) => mem :: replace (attrs , new) , Item :: Verbatim (_) => Vec :: new () , } } }
};
}
