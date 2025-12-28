macro_rules! Field {
    () => {
        # [doc = " A description of a struct or enum field."] # [derive (Clone , Copy , Debug , PartialEq , Eq , PartialOrd , Ord)] # [cfg (feature = "details")] pub struct Field { # [doc = " The name of the field being described."] pub name : & 'static str , # [doc = " The total size of the field."] pub size : usize , # [doc = " How many items this collection is holding, if it is one."] pub collection_items : Option < usize > , }
    };
}

Field!();