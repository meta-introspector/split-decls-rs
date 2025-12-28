macro_rules! PatMigration {
    () => {
        # [doc = " For patterns flagged for migration during HIR typeck, this handles constructing and emitting"] # [doc = " a diagnostic suggestion."] pub (super) struct PatMigration < 'a > { suggestion : Vec < (Span , String) > , ref_pattern_count : usize , binding_mode_count : usize , # [doc = " Internal state: the ref-mutability of the default binding mode at the subpattern being"] # [doc = " lowered, with the span where it was introduced. `None` for a by-value default mode."] default_mode_span : Option < (Span , ty :: Mutability) > , # [doc = " Labels for where incompatibility-causing by-ref default binding modes were introduced."] default_mode_labels : FxIndexMap < Span , Mutability > , # [doc = " Information collected from typeck, including spans for subpatterns invalid in Rust 2024."] info : & 'a Rust2024IncompatiblePatInfo , }
    };
}

PatMigration!()