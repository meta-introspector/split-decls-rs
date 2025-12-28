macro_rules! DeclarationItem {
    () => {
        # [derive (Debug , Clone , Serialize , Deserialize)] pub enum DeclarationItem { Const (String) , Struct (String) , Enum (String) , Fn (String) , Static (String) , Macro (String) , Mod (String) , Trait (String) , TraitAlias (String) , Type (String) , Union (String) , Other (String) , }
    };
}

DeclarationItem!();