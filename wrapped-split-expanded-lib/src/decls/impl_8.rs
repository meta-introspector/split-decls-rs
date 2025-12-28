macro_rules! deps {
    () => {
        DeclarationItem!();
        Declaration!();
        SerializableDeclaration!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl From < Declaration > for SerializableDeclaration { fn from (decl : Declaration) -> Self { SerializableDeclaration { identifier : decl . get_identifier () , declaration_type : match & decl . item { DeclarationItem :: Const (_) => "const" . to_string () , DeclarationItem :: Struct (_) => "struct" . to_string () , DeclarationItem :: Enum (_) => "enum" . to_string () , DeclarationItem :: Fn (_) => "function" . to_string () , DeclarationItem :: Static (_) => "static" . to_string () , DeclarationItem :: Macro (_) => "macro" . to_string () , DeclarationItem :: Mod (_) => "module" . to_string () , DeclarationItem :: Trait (_) => "trait" . to_string () , DeclarationItem :: TraitAlias (_) => "trait_alias" . to_string () , DeclarationItem :: Type (_) => "type_alias" . to_string () , DeclarationItem :: Union (_) => "union" . to_string () , DeclarationItem :: Other (_) => "other" . to_string () , } , referenced_types : decl . referenced_types , referenced_functions : decl . referenced_functions , external_identifiers : decl . external_identifiers , source_file : decl . source_file , crate_name : decl . crate_name , resolved_dependencies : decl . resolved_dependencies , is_proc_macro : decl . is_proc_macro , required_imports : decl . required_imports , direct_dependencies : decl . direct_dependencies , extern_crates : decl . extern_crates , is_public : decl . is_public , } } }
    };
}

impl_8!()