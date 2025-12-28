macro_rules! RenameRule {
    () => {
        # [doc = " The different possible ways to change case of fields in a struct, or variants in an enum."] # [derive (Copy , Clone , PartialEq)] pub enum RenameRule { # [doc = " Don't apply a default rename rule."] None , # [doc = " Rename direct children to \"lowercase\" style."] LowerCase , # [doc = " Rename direct children to \"UPPERCASE\" style."] UpperCase , # [doc = " Rename direct children to \"PascalCase\" style, as typically used for"] # [doc = " enum variants."] PascalCase , # [doc = " Rename direct children to \"camelCase\" style."] CamelCase , # [doc = " Rename direct children to \"snake_case\" style, as commonly used for"] # [doc = " fields."] SnakeCase , # [doc = " Rename direct children to \"SCREAMING_SNAKE_CASE\" style, as commonly"] # [doc = " used for constants."] ScreamingSnakeCase , # [doc = " Rename direct children to \"kebab-case\" style."] KebabCase , # [doc = " Rename direct children to \"SCREAMING-KEBAB-CASE\" style."] ScreamingKebabCase , }
    };
}

RenameRule!();