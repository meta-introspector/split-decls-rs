// Generated macro for Id (struct)
macro_rules! Depcrate_idId {
() => {
// Module: crate::id
// Provides: {"Id"}
// Dependencies: {}
# [doc = " The `Id` of a salsa struct in the database [`Table`](`crate::table::Table`)."] # [doc = ""] # [doc = " The high-order bits of an `Id` store a 32-bit generation counter, while"] # [doc = " the low-order bits pack a [`PageIndex`](`crate::table::PageIndex`) and"] # [doc = " [`SlotIndex`](`crate::table::SlotIndex`) within the page."] # [doc = ""] # [doc = " The low-order bits of `Id` are a `u32` ranging from `0..Id::MAX_U32`."] # [doc = " The maximum range is smaller than a standard `u32` to leave"] # [doc = " room for niches; currently there is only one niche, so that"] # [doc = " `Option<Id>` is the same size as an `Id`."] # [doc = ""] # [doc = " As an end-user of `Salsa` you will generally not use `Id` directly,"] # [doc = " it is wrapped in new types."] # [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] pub struct Id { index : NonZeroU32 , generation : u32 , }
};
}
