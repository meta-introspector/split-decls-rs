macro_rules! doc_fake_variadic_is_allowed_self_ty {
    () => {
        fn doc_fake_variadic_is_allowed_self_ty (self_ty : & hir :: Ty < '_ >) -> bool { matches ! (& self_ty . kind , hir :: TyKind :: Tup ([_])) || if let hir :: TyKind :: FnPtr (fn_ptr_ty) = & self_ty . kind { fn_ptr_ty . decl . inputs . len () == 1 } else { false } || (if let hir :: TyKind :: Path (hir :: QPath :: Resolved (_ , path)) = & self_ty . kind && let Some (& [hir :: GenericArg :: Type (ty)]) = path . segments . last () . map (| last | last . args () . args) { doc_fake_variadic_is_allowed_self_ty (ty . as_unambig_ty ()) } else { false }) }
    };
}

doc_fake_variadic_is_allowed_self_ty!();