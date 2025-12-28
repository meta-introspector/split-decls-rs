macro_rules! deps {
    () => {
        Declaration!();
    };
}

macro_rules! DeclsVisitor {
    () => {
        deps!();
        pub struct DeclsVisitor < 'a > { pub declarations : HashMap < String , Declaration > , pub fn_count : usize , pub struct_count : usize , pub enum_count : usize , pub const_count : usize , pub static_count : usize , pub macro_count : usize , pub mod_count : usize , pub trait_count : usize , pub trait_alias_count : usize , pub type_count : usize , pub union_count : usize , pub other_item_count : usize , pub source_file : PathBuf , pub crate_name : String , pub verbosity : u8 , pub file_extern_crates : HashSet < String > , pub warnings : & 'a mut Vec < String > , }
    };
}

DeclsVisitor!()