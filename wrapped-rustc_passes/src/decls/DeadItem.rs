macro_rules! DeadItem {
    () => {
        struct DeadItem { def_id : LocalDefId , name : Symbol , level : (lint :: Level , Option < LintExpectationId >) , }
    };
}

DeadItem!();