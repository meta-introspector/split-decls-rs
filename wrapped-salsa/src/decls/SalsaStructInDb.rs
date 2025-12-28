macro_rules! deps {
    () => {
        Id!();
        IngredientIndices!();
        Revision!();
        IngredientIndex!();
        DatabaseKeyIndex!();
        MemoIngredientMap!();
        Zalsa!();
        Lookup!();
    };
}

macro_rules! SalsaStructInDb {
    () => {
        deps!();
        pub trait SalsaStructInDb : Sized { type MemoIngredientMap : MemoIngredientMap ; # [doc = " Lookup or create ingredient indices."] # [doc = ""] # [doc = " Note that this method does *not* create the ingredients themselves, this is handled by"] # [doc = " [`crate::zalsa::JarEntry::get_or_create`]. This method only creates"] # [doc = " or looks up the indices corresponding to the ingredients."] # [doc = ""] # [doc = " While implementors of this trait may call [`crate::zalsa::JarEntry::get_or_create`]"] # [doc = " to create the ingredient, they aren't required to. For example, supertypes recursively"] # [doc = " call [`crate::zalsa::JarEntry::get_or_create`] for their variants and combine them."] fn lookup_ingredient_index (zalsa : & Zalsa) -> IngredientIndices ; # [doc = " Returns the IDs of any instances of this struct in the database."] fn entries (zalsa : & Zalsa) -> impl Iterator < Item = DatabaseKeyIndex > + '_ ; # [doc = " Plumbing to support nested salsa supertypes."] # [doc = ""] # [doc = " In the example below, there are two supertypes: `InnerEnum` and `OuterEnum`,"] # [doc = " where the former is a supertype of `Input` and `Interned1` and the latter"] # [doc = " is a supertype of `InnerEnum` and `Interned2`."] # [doc = ""] # [doc = " ```ignore"] # [doc = " #[salsa::input]"] # [doc = " struct Input {}"] # [doc = ""] # [doc = " #[salsa::interned]"] # [doc = " struct Interned1 {}"] # [doc = ""] # [doc = " #[salsa::interned]"] # [doc = " struct Interned2 {}"] # [doc = ""] # [doc = " #[derive(Debug, salsa::Enum)]"] # [doc = " enum InnerEnum {"] # [doc = "     Input(Input),"] # [doc = "     Interned1(Interned1),"] # [doc = " }"] # [doc = ""] # [doc = " #[derive(Debug, salsa::Enum)]"] # [doc = " enum OuterEnum {"] # [doc = "     InnerEnum(InnerEnum),"] # [doc = "     Interned2(Interned2),"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Imagine `OuterEnum` got a [`salsa::Id`][Id] and it wants to know which variant it belongs to."] # [doc = ""] # [doc = " `OuterEnum` cannot ask each variant \"what is your ingredient index?\" and compare because `InnerEnum`"] # [doc = " has *multiple*, possible ingredient indices. Alternatively, `OuterEnum` could ask eaach variant"] # [doc = " \"is this value yours?\" and then invoke [`FromId`][crate::id::FromId] with the correct variant,"] # [doc = " but this duplicates work: now, `InnerEnum` will have to repeat this check-and-cast for *its*"] # [doc = " variants."] # [doc = ""] # [doc = " Instead, the implementor keeps track of the [`std::any::TypeId`] of the ID struct, and ask each"] # [doc = " variant to \"cast\" to it. If it succeeds, `cast` returns that value; if not, we"] # [doc = " go to the next variant."] # [doc = ""] # [doc = " Why `TypeId` and not `IngredientIndex`? Because it's cheaper and easier: the `TypeId` is readily"] # [doc = " available at compile time, while the `IngredientIndex` requires a runtime lookup."] fn cast (id : Id , type_id : TypeId) -> Option < Self > ; # [doc = " Return the memo table associated with `id`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The parameter `current_revision` must be the current revision of the owner of database"] # [doc = " owning this table."] unsafe fn memo_table (zalsa : & Zalsa , id : Id , current_revision : Revision ,) -> MemoTableWithTypes < '_ > ; }
    };
}

SalsaStructInDb!()