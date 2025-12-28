macro_rules! deps {
    () => {
        BranchInfo!();
        NotInfo!();
        CoverageInfoBuilder!();
        Scope!();
        CFG!();
        BlockMarkerGen!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl CoverageInfoBuilder { # [doc = " Creates a new coverage info builder, but only if coverage instrumentation"] # [doc = " is enabled and `def_id` represents a function that is eligible for coverage."] pub (crate) fn new_if_enabled (tcx : TyCtxt < '_ > , def_id : LocalDefId) -> Option < Self > { if ! tcx . sess . instrument_coverage () || ! tcx . is_eligible_for_coverage (def_id) { return None ; } Some (Self { nots : FxHashMap :: default () , markers : BlockMarkerGen :: default () , branch_info : tcx . sess . instrument_coverage_branch () . then (BranchInfo :: default) , }) } # [doc = " Unary `!` expressions inside an `if` condition are lowered by lowering"] # [doc = " their argument instead, and then reversing the then/else arms of that `if`."] # [doc = ""] # [doc = " That's awkward for branch coverage instrumentation, so to work around that"] # [doc = " we pre-emptively visit any affected `!` expressions, and record extra"] # [doc = " information that [`Builder::visit_coverage_branch_condition`] can use to"] # [doc = " synthesize branch instrumentation for the enclosing `!`."] pub (crate) fn visit_unary_not (& mut self , thir : & Thir < '_ > , unary_not : ExprId) { assert_matches ! (thir [unary_not] . kind , ExprKind :: Unary { op : UnOp :: Not , .. }) ; if self . branch_info . is_none () { return ; } self . visit_with_not_info (thir , unary_not , NotInfo { enclosing_not : unary_not , is_flipped : false } ,) ; } fn visit_with_not_info (& mut self , thir : & Thir < '_ > , expr_id : ExprId , not_info : NotInfo) { match self . nots . entry (expr_id) { Entry :: Occupied (_) => return , Entry :: Vacant (entry) => entry . insert (not_info) , } ; match thir [expr_id] . kind { ExprKind :: Unary { op : UnOp :: Not , arg } => { let not_info = NotInfo { is_flipped : ! not_info . is_flipped , .. not_info } ; self . visit_with_not_info (thir , arg , not_info) ; } ExprKind :: Scope { value , .. } => self . visit_with_not_info (thir , value , not_info) , ExprKind :: Use { source } => self . visit_with_not_info (thir , source , not_info) , _ => { } } } fn register_two_way_branch < 'tcx > (& mut self , cfg : & mut CFG < 'tcx > , source_info : SourceInfo , true_block : BasicBlock , false_block : BasicBlock ,) { let Some (branch_info) = self . branch_info . as_mut () else { return } ; let true_marker = self . markers . inject_block_marker (cfg , source_info , true_block) ; let false_marker = self . markers . inject_block_marker (cfg , source_info , false_block) ; branch_info . branch_spans . push (BranchSpan { span : source_info . span , true_marker , false_marker , }) ; } pub (crate) fn into_done (self) -> Box < CoverageInfoHi > { let Self { nots : _ , markers : BlockMarkerGen { num_block_markers } , branch_info } = self ; let branch_spans = branch_info . map (| branch_info | branch_info . branch_spans) . unwrap_or_default () ; Box :: new (CoverageInfoHi { num_block_markers , branch_spans }) } pub (crate) fn as_done (& self) -> Box < CoverageInfoHi > { let & Self { nots : _ , markers : BlockMarkerGen { num_block_markers } , ref branch_info } = self ; let branch_spans = branch_info . as_ref () . map (| branch_info | branch_info . branch_spans . as_slice ()) . unwrap_or_default () . to_owned () ; Box :: new (CoverageInfoHi { num_block_markers , branch_spans }) } }
    };
}

impl_39!();