macro_rules! deps {
    () => {
        Metadata!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl PartialEq for Metadata < '_ > { # [inline] fn eq (& self , other : & Self) -> bool { if core :: ptr :: eq (self , other) { true } else if cfg ! (not (debug_assertions)) { self . callsite () == other . callsite () } else { let Metadata { name : lhs_name , target : lhs_target , level : lhs_level , module_path : lhs_module_path , file : lhs_file , line : lhs_line , fields : lhs_fields , kind : lhs_kind , } = self ; let Metadata { name : rhs_name , target : rhs_target , level : rhs_level , module_path : rhs_module_path , file : rhs_file , line : rhs_line , fields : rhs_fields , kind : rhs_kind , } = & other ; self . callsite () == other . callsite () && lhs_name == rhs_name && lhs_target == rhs_target && lhs_level == rhs_level && lhs_module_path == rhs_module_path && lhs_file == rhs_file && lhs_line == rhs_line && lhs_fields == rhs_fields && lhs_kind == rhs_kind } } }
    };
}

impl_198!();