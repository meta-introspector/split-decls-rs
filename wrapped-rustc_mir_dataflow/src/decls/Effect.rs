macro_rules! Effect {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq , Eq , PartialOrd , Ord)] enum Effect { # [doc = " The \"early\" effect (e.g., `apply_early_statement_effect`) for a statement/terminator."] Early , # [doc = " The \"primary\" effect (e.g., `apply_primary_statement_effect`) for a statement/terminator."] Primary , }
    };
}

Effect!();