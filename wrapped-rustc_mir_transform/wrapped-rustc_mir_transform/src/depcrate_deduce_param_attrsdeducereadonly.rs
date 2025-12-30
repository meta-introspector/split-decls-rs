// Generated macro for DeduceReadOnly (struct)
macro_rules! Depcrate_deduce_param_attrsDeduceReadOnly {
() => {
// Module: crate::deduce_param_attrs
// Provides: {"DeduceReadOnly"}
// Dependencies: {}
# [doc = " A visitor that determines which arguments have been mutated. We can't use the mutability field"] # [doc = " on LocalDecl for this because it has no meaning post-optimization."] struct DeduceReadOnly { # [doc = " Each bit is indexed by argument number, starting at zero (so 0 corresponds to local decl"] # [doc = " 1). The bit is true if the argument may have been mutated or false if we know it hasn't"] # [doc = " been up to the point we're at."] mutable_args : DenseBitSet < usize > , }
};
}
