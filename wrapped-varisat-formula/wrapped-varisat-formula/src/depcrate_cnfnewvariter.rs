// Generated macro for NewVarIter (struct)
macro_rules! Depcrate_cnfNewVarIter {
() => {
// Module: crate::cnf
// Provides: {"NewVarIter"}
// Dependencies: {}
# [doc = " Iterator over new variables or literals."] # [doc = ""] # [doc = " Created by the [`new_var_iter`][ExtendFormula::new_var_iter] and"] # [doc = " [`new_lit_iter`][ExtendFormula::new_lit_iter] methods of [`ExtendFormula`]."] pub struct NewVarIter < 'a , F , V = Var > { formula : & 'a mut F , vars_left : usize , phantom : std :: marker :: PhantomData < V > , }
};
}
