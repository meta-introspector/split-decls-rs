macro_rules! deps {
    () => {
        RevealedTy!();
        Overlap!();
        OverlappingRangeEndpoints!();
        MixedDerefPatternConstructors!();
        PatCx!();
        IntRange!();
        Constructor!();
        RustcPatCtxt!();
        GappedRange!();
        ExclusiveRangeMissingGap!();
        RangeEnd!();
        ExclusiveRangeMissingMax!();
        ConstructorSet!();
        PrivateUninhabitedField!();
        DeconstructedPat!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl < 'p , 'tcx : 'p > PatCx for RustcPatCtxt < 'p , 'tcx > { type Ty = RevealedTy < 'tcx > ; type Error = ErrorGuaranteed ; type VariantIdx = VariantIdx ; type StrLit = ty :: Value < 'tcx > ; type ArmData = HirId ; type PatData = & 'p Pat < 'tcx > ; fn is_exhaustive_patterns_feature_on (& self) -> bool { self . tcx . features () . exhaustive_patterns () } fn ctor_arity (& self , ctor : & crate :: constructor :: Constructor < Self > , ty : & Self :: Ty) -> usize { self . ctor_arity (ctor , * ty) } fn ctor_sub_tys (& self , ctor : & crate :: constructor :: Constructor < Self > , ty : & Self :: Ty ,) -> impl Iterator < Item = (Self :: Ty , PrivateUninhabitedField) > + ExactSizeIterator { self . ctor_sub_tys (ctor , * ty) } fn ctors_for_ty (& self , ty : & Self :: Ty ,) -> Result < crate :: constructor :: ConstructorSet < Self > , Self :: Error > { self . ctors_for_ty (* ty) } fn write_variant_name (f : & mut fmt :: Formatter < '_ > , ctor : & crate :: constructor :: Constructor < Self > , ty : & Self :: Ty ,) -> fmt :: Result { if let ty :: Adt (adt , _) = ty . kind () { let variant = adt . variant (Self :: variant_index_for_adt (ctor , * adt)) ; write ! (f , "{}" , variant . name) ? ; } Ok (()) } fn bug (& self , fmt : fmt :: Arguments < '_ >) -> Self :: Error { span_bug ! (self . scrut_span , "{}" , fmt) } fn lint_overlapping_range_endpoints (& self , pat : & crate :: pat :: DeconstructedPat < Self > , overlaps_on : IntRange , overlaps_with : & [& crate :: pat :: DeconstructedPat < Self >] ,) { let overlap_as_pat = self . print_pat_range (& overlaps_on , * pat . ty ()) ; let overlaps : Vec < _ > = overlaps_with . iter () . map (| pat | pat . data () . span) . map (| span | errors :: Overlap { range : overlap_as_pat . to_string () , span }) . collect () ; let pat_span = pat . data () . span ; self . tcx . emit_node_span_lint (lint :: builtin :: OVERLAPPING_RANGE_ENDPOINTS , self . match_lint_level , pat_span , errors :: OverlappingRangeEndpoints { overlap : overlaps , range : pat_span } ,) ; } fn complexity_exceeded (& self) -> Result < () , Self :: Error > { let span = self . whole_match_span . unwrap_or (self . scrut_span) ; Err (self . tcx . dcx () . span_err (span , "reached pattern complexity limit")) } fn lint_non_contiguous_range_endpoints (& self , pat : & crate :: pat :: DeconstructedPat < Self > , gap : IntRange , gapped_with : & [& crate :: pat :: DeconstructedPat < Self >] ,) { let & thir_pat = pat . data () ; let thir :: PatKind :: Range (range) = & thir_pat . kind else { return } ; if range . end != rustc_hir :: RangeEnd :: Excluded { return ; } let suggested_range : String = { let mut suggested_range = PatRange :: clone (range) ; suggested_range . end = rustc_hir :: RangeEnd :: Included ; suggested_range . to_string () } ; let gap_as_pat = self . print_pat_range (& gap , * pat . ty ()) ; if gapped_with . is_empty () { self . tcx . emit_node_span_lint (lint :: builtin :: NON_CONTIGUOUS_RANGE_ENDPOINTS , self . match_lint_level , thir_pat . span , errors :: ExclusiveRangeMissingMax { first_range : thir_pat . span , max : gap_as_pat , suggestion : suggested_range , } ,) ; } else { self . tcx . emit_node_span_lint (lint :: builtin :: NON_CONTIGUOUS_RANGE_ENDPOINTS , self . match_lint_level , thir_pat . span , errors :: ExclusiveRangeMissingGap { first_range : thir_pat . span , gap : gap_as_pat . to_string () , suggestion : suggested_range , gap_with : gapped_with . iter () . map (| pat | errors :: GappedRange { span : pat . data () . span , gap : gap_as_pat . to_string () , first_range : range . to_string () , }) . collect () , } ,) ; } } fn match_may_contain_deref_pats (& self) -> bool { self . internal_state . has_lowered_deref_pat . get () } fn report_mixed_deref_pat_ctors (& self , deref_pat : & crate :: pat :: DeconstructedPat < Self > , normal_pat : & crate :: pat :: DeconstructedPat < Self > ,) -> Self :: Error { let deref_pattern_label = deref_pat . data () . span ; let normal_constructor_label = normal_pat . data () . span ; self . tcx . dcx () . emit_err (errors :: MixedDerefPatternConstructors { spans : vec ! [deref_pattern_label , normal_constructor_label] , smart_pointer_ty : deref_pat . ty () . inner () , deref_pattern_label , normal_constructor_label , }) } }
    };
}

impl_82!()