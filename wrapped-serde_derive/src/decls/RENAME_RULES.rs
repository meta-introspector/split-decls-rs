macro_rules! deps {
    () => {
        RenameRule!();
    };
}

macro_rules! RENAME_RULES {
    () => {
        deps!();
        static RENAME_RULES : & [(& str , RenameRule)] = & [("lowercase" , LowerCase) , ("UPPERCASE" , UpperCase) , ("PascalCase" , PascalCase) , ("camelCase" , CamelCase) , ("snake_case" , SnakeCase) , ("SCREAMING_SNAKE_CASE" , ScreamingSnakeCase) , ("kebab-case" , KebabCase) , ("SCREAMING-KEBAB-CASE" , ScreamingKebabCase) ,] ;
    };
}

RENAME_RULES!();