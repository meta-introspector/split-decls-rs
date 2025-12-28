macro_rules! is_call_like {
    () => {
        # [doc = " A terminator that's more call-like (might do a bunch of work, might panic, etc)"] # [doc = " than it is goto-/return-like (no side effects, etc)."] # [doc = ""] # [doc = " Used to treat multi-call functions (which could inline exponentially)"] # [doc = " different from those that only do one or none of these \"complex\" things."] pub (super) fn is_call_like (terminator : & Terminator < '_ >) -> bool { use TerminatorKind :: * ; match terminator . kind { Call { .. } | TailCall { .. } | Drop { .. } | Assert { .. } | InlineAsm { .. } => true , Goto { .. } | SwitchInt { .. } | UnwindResume | UnwindTerminate (_) | Return | Unreachable => false , Yield { .. } | CoroutineDrop | FalseEdge { .. } | FalseUnwind { .. } => { unreachable ! () } } }
    };
}

is_call_like!()