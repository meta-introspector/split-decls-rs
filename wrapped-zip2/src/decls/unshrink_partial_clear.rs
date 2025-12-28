macro_rules! deps {
    () => {
        CodeQueue!();
        Codetab!();
    };
}

macro_rules! unshrink_partial_clear {
    () => {
        deps!();
        fn unshrink_partial_clear (codetab : & mut [Codetab] , queue : & mut CodeQueue) { let mut is_prefix = [false ; MAX_CODE + 1] ; for code in codetab . iter () . take (MAX_CODE + 1) . skip (CONTROL_CODE + 1) { if let Some (prefix_code) = code . prefix_code { is_prefix [prefix_code as usize] = true ; } } let mut code_queue_size = 0 ; for i in (CONTROL_CODE + 1) .. MAX_CODE { if ! is_prefix [i] { codetab [i] . prefix_code = None ; queue . codes [code_queue_size] = Some (i as u16) ; code_queue_size += 1 ; } } queue . codes [code_queue_size] = None ; queue . next_idx = 0 ; }
    };
}

unshrink_partial_clear!();