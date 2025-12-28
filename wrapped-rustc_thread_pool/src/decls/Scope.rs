macro_rules! deps {
    () => {
        ScopeBase!();
    };
}

macro_rules! Scope {
    () => {
        deps!();
        # [doc = " Represents a fork-join scope which can be used to spawn any number of tasks."] # [doc = " See [`scope()`] for more information."] # [doc = ""] # [doc = "[`scope()`]: fn.scope.html"] pub struct Scope < 'scope > { base : ScopeBase < 'scope > , }
    };
}

Scope!();