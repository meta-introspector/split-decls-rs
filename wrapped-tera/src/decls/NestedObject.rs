macro_rules! NestedObject {
    () => {
        # [allow (dead_code)] # [derive (Debug , Serialize)] pub struct NestedObject { pub label : String , pub parent : Option < Box < NestedObject > > , pub numbers : Vec < usize > , }
    };
}

NestedObject!();