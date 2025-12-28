macro_rules! Report {
    () => {
        struct Report { failures : usize , created_wip : usize , }
    };
}

Report!();