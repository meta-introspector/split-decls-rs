macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Default for Config { fn default () -> Self { Self { ansi : true , indent_lines : false , indent_amount : 2 , targets : false , render_thread_ids : false , render_thread_names : false , wraparound : usize :: MAX , verbose_entry : false , verbose_exit : false , span_retrace : false , bracketed_fields : false , deferred_spans : false , span_modes : false , } } }
    };
}

impl_10!()