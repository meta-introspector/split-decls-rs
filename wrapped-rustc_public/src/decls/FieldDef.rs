macro_rules! deps {
    () => {
        DefId!();
        Symbol!();
    };
}

macro_rules! FieldDef {
    () => {
        deps!();
        # [derive (Clone , Debug , Eq , PartialEq , Serialize)] pub struct FieldDef { # [doc = " The field definition."] # [doc = ""] # [doc = " ## Warning"] # [doc = " Do not access this field directly! This is public for the compiler to have access to it."] pub def : DefId , # [doc = " The field name."] pub name : Symbol , }
    };
}

FieldDef!();