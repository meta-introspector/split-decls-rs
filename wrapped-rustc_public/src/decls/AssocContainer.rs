macro_rules! AssocContainer {
    () => {
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub enum AssocContainer { InherentImpl , # [doc = " The `AssocDef` points to the trait item being implemented."] TraitImpl (AssocDef) , Trait , }
    };
}

AssocContainer!()