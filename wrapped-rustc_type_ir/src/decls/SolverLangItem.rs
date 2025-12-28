macro_rules! SolverLangItem {
    () => {
        # [doc = " Lang items used by the new trait solver. This can be mapped to whatever internal"] # [doc = " representation of `LangItem`s used in the underlying compiler implementation."] pub enum SolverLangItem { AsyncFnKindUpvars , AsyncFnOnceOutput , CallOnceFuture , CallRefFuture , CoroutineReturn , CoroutineYield , DynMetadata , FutureOutput , Metadata , }
    };
}

SolverLangItem!();