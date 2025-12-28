macro_rules! deps {
    () => {
        SafetyContext!();
        UnusedUnsafeWarning!();
    };
}

macro_rules! UnsafetyVisitor {
    () => {
        deps!();
        struct UnsafetyVisitor < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , thir : & 'a Thir < 'tcx > , # [doc = " The `HirId` of the current scope, which would be the `HirId`"] # [doc = " of the current HIR node, modulo adjustments. Used for lint levels."] hir_context : HirId , # [doc = " The current \"safety context\". This notably tracks whether we are in an"] # [doc = " `unsafe` block, and whether it has been used."] safety_context : SafetyContext , # [doc = " The `#[target_feature]` attributes of the body. Used for checking"] # [doc = " calls to functions with `#[target_feature]` (RFC 2396)."] body_target_features : & 'tcx [TargetFeature] , # [doc = " When inside the LHS of an assignment to a field, this is the type"] # [doc = " of the LHS and the span of the assignment expression."] assignment_info : Option < Ty < 'tcx > > , in_union_destructure : bool , typing_env : ty :: TypingEnv < 'tcx > , inside_adt : bool , warnings : & 'a mut Vec < UnusedUnsafeWarning > , # [doc = " Flag to ensure that we only suggest wrapping the entire function body in"] # [doc = " an unsafe block once."] suggest_unsafe_block : bool , }
    };
}

UnsafetyVisitor!();