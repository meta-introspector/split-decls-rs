macro_rules! deps {
    () => {
        DependencyVisitor!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < 'ast > Visit < 'ast > for DependencyVisitor { fn visit_path (& mut self , path : & 'ast syn :: Path) { let path_str = quote ! { # path } . to_string () ; self . required_imports . insert (path_str) ; visit :: visit_path (self , path) ; } fn visit_item (& mut self , i : & 'ast syn :: Item) { visit :: visit_item (self , i) ; } fn visit_trait_item (& mut self , i : & 'ast syn :: TraitItem) { visit :: visit_trait_item (self , i) ; } fn visit_type (& mut self , i : & 'ast syn :: Type) { let type_str = quote ! { # i } . to_string () ; self . required_imports . insert (type_str) ; visit :: visit_type (self , i) ; } fn visit_expr (& mut self , i : & 'ast syn :: Expr) { let expr_str = quote ! { # i } . to_string () ; self . required_imports . insert (expr_str) ; visit :: visit_expr (self , i) ; } }
    };
}

impl_17!()