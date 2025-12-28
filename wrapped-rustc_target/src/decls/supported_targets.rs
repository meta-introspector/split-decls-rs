macro_rules! deps {
    () => {
        Target!();
    };
}

macro_rules! supported_targets {
    () => {
        deps!();
        macro_rules ! supported_targets { ($ (($ tuple : literal , $ module : ident) ,) +) => { mod targets { $ (pub (crate) mod $ module ;) + } # [doc = " List of supported targets"] pub static TARGETS : & [& str] = & [$ ($ tuple) ,+] ; fn load_builtin (target : & str) -> Option < Target > { let t = match target { $ ($ tuple => targets ::$ module :: target () ,) + _ => return None , } ; debug ! ("got builtin target: {:?}" , t) ; Some (t) } fn load_all_builtins () -> impl Iterator < Item = Target > { [$ (targets ::$ module :: target ,) +] . into_iter () . map (| f | f ()) } # [cfg (test)] mod tests { $ (# [test] fn $ module () { crate :: spec :: targets ::$ module :: target () . test_target () }) + } } ; }
    };
}

supported_targets!()