macro_rules! local_decls_for_sig {
    () => {
        fn local_decls_for_sig < 'tcx > (sig : & ty :: FnSig < 'tcx > , span : Span ,) -> IndexVec < Local , LocalDecl < 'tcx > > { iter :: once (LocalDecl :: new (sig . output () , span)) . chain (sig . inputs () . iter () . map (| ity | LocalDecl :: new (* ity , span) . immutable ())) . collect () }
    };
}

local_decls_for_sig!();