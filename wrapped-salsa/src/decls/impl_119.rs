macro_rules! deps {
    () => {
        SalsaStructInDb!();
        Id!();
        MemoIngredientIndex!();
        IngredientImpl!();
        IngredientIndex!();
        MemoIngredientMap!();
        Configuration!();
        DatabaseKeyIndex!();
        Memo!();
        Zalsa!();
        DatabaseDownCaster!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < C > IngredientImpl < C > where C : Configuration , { pub fn new (index : IngredientIndex , memo_ingredient_indices : < C :: SalsaStruct < 'static > as SalsaStructInDb > :: MemoIngredientMap , lru : usize ,) -> Self { Self { index , memo_ingredient_indices , lru : lru :: Lru :: new (lru) , deleted_entries : Default :: default () , view_caster : OnceLock :: new () , sync_table : SyncTable :: new (index) , } } # [doc = " Set the view-caster for this tracked function ingredient, if it has"] # [doc = " not already been initialized."] # [inline] pub fn get_or_init (& self , view_caster : impl FnOnce () -> DatabaseDownCaster < C :: DbView > ,) -> & Self { self . view_caster . get_or_init (view_caster) ; self } # [inline] pub fn database_key_index (& self , key : Id) -> DatabaseKeyIndex { DatabaseKeyIndex :: new (self . index , key) } pub fn set_capacity (& mut self , capacity : usize) { self . lru . set_capacity (capacity) ; } # [doc = " Returns a reference to the memo value that lives as long as self."] # [doc = " This is UNSAFE: the caller is responsible for ensuring that the"] # [doc = " memo will not be released so long as the `&self` is valid."] # [doc = " This is done by (a) ensuring the memo is present in the memo-map"] # [doc = " when this function is called and (b) ensuring that any entries"] # [doc = " removed from the memo-map are added to `deleted_entries`, which is"] # [doc = " only cleared with `&mut self`."] unsafe fn extend_memo_lifetime < 'this > (& 'this self , memo : & memo :: Memo < 'this , C > ,) -> & 'this memo :: Memo < 'this , C > { unsafe { std :: mem :: transmute (memo) } } fn insert_memo < 'db > (& 'db self , zalsa : & 'db Zalsa , id : Id , mut memo : memo :: Memo < 'db , C > , memo_ingredient_index : MemoIngredientIndex ,) -> & 'db memo :: Memo < 'db , C > { if let Some (tracked_struct_ids) = memo . revisions . tracked_struct_ids_mut () { tracked_struct_ids . shrink_to_fit () ; } let memo = NonNull :: from (Box :: leak (Box :: new (memo))) ; if let Some (old_value) = self . insert_memo_into_table_for (zalsa , id , memo , memo_ingredient_index) { unsafe { self . deleted_entries . push (old_value) } ; } unsafe { self . extend_memo_lifetime (memo . as_ref ()) } } # [inline] fn memo_ingredient_index (& self , zalsa : & Zalsa , id : Id) -> MemoIngredientIndex { self . memo_ingredient_indices . get_zalsa_id (zalsa , id) } fn view_caster (& self) -> & DatabaseDownCaster < C :: DbView > { self . view_caster . get () . expect ("tracked function ingredients cannot be accessed before calling `init`") } }
    };
}

impl_119!()