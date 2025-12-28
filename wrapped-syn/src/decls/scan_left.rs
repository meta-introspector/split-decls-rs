macro_rules! deps {
    () => {
        FixupContext!();
        Precedence!();
    };
}

macro_rules! scan_left {
    () => {
        deps!();
        # [cfg (feature = "full")] fn scan_left (expr : & Expr , fixup : FixupContext) -> bool { match expr { Expr :: Assign (_) => fixup . previous_operator <= Precedence :: Assign , Expr :: Binary (e) => match Precedence :: of_binop (& e . op) { Precedence :: Assign => fixup . previous_operator <= Precedence :: Assign , binop_prec => fixup . previous_operator < binop_prec , } , Expr :: Cast (_) => fixup . previous_operator < Precedence :: Cast , Expr :: Range (e) => e . start . is_none () || fixup . previous_operator < Precedence :: Assign , _ => true , } }
    };
}

scan_left!()