// Generated macro for related (function)
macro_rules! Depcrate_general_categoryrelated {
() => {
// Module: crate::general_category
// Provides: {"related"}
// Dependencies: {}
# [doc = " Related returns a set of sets of codepoints corresponding to the \"related\""] # [doc = " groups of categories defined by Table 12 in UAX#44 S5.7.1."] # [doc = ""] # [doc = " The given `cats` should correspond to the normal set of general categories,"] # [doc = " keyed by canonical name."] fn related (propvals : & PropertyValues , cats : & BTreeMap < String , BTreeSet < u32 > > ,) -> BTreeMap < String , BTreeSet < u32 > > { let mut sets = BTreeMap :: new () ; for (name , components) in related_categories (propvals) { let set = sets . entry (name) . or_insert (BTreeSet :: new ()) ; for component in components { set . extend (cats [& component] . iter () . cloned ()) ; } } sets }
};
}
