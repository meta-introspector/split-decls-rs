macro_rules! AssembleCandidatesFrom {
    () => {
        # [doc = " Allows callers of `assemble_and_evaluate_candidates` to choose whether to limit"] # [doc = " candidate assembly to param-env and alias-bound candidates."] # [doc = ""] # [doc = " On top of being a micro-optimization, as it avoids doing unnecessary work when"] # [doc = " a param-env trait bound candidate shadows impls for normalization, this is also"] # [doc = " required to prevent query cycles due to RPITIT inference. See the issue at:"] # [doc = " <https://github.com/rust-lang/trait-system-refactor-initiative/issues/173>."] pub (super) enum AssembleCandidatesFrom { All , # [doc = " Only assemble candidates from the environment and alias bounds, ignoring"] # [doc = " user-written and built-in impls. We only expect `ParamEnv` and `AliasBound`"] # [doc = " candidates to be assembled."] EnvAndBounds , }
    };
}

AssembleCandidatesFrom!();