macro_rules! deps {
    () => {
        MacroDefinition!();
    };
}

macro_rules! MacroDefinitionMap {
    () => {
        deps!();
        # [doc = " Maps { macro => macro_definition }"] pub type MacroDefinitionMap = HashMap < String , MacroDefinition > ;
    };
}

MacroDefinitionMap!()