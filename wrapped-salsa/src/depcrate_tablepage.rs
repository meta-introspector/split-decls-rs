// Generated macro for Page (struct)
macro_rules! Depcrate_tablePage {
() => {
// Module: crate::table
// Provides: {"Page"}
// Dependencies: {}
struct Page { # [doc = " The ingredient for elements on this page."] ingredient : IngredientIndex , # [doc = " Number of elements of `data` that are initialized."] allocated : AtomicUsize , # [doc = " The potentially uninitialized data of this page. As we initialize new entries, we increment `allocated`."] # [doc = " This is a box allocated `PageData<SlotType>`"] data : NonNull < () > , # [doc = " A vtable for the slot type stored in this page."] slot_vtable : & 'static SlotVTable , # [doc = " The type id of what is stored as entries in data."] slot_type_id : TypeId , memo_types : Arc < MemoTableTypes > , }
};
}
