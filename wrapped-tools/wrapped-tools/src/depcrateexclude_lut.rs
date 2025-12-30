// Generated macro for EXCLUDE_LUT (static)
macro_rules! DepcrateEXCLUDE_LUT {
() => {
// Module: crate
// Provides: {"EXCLUDE_LUT"}
// Dependencies: {}
static EXCLUDE_LUT : LazyLock < Mutex < Option < gix_worktree :: Stack > > > = LazyLock :: new (| | { let cache = (| | { let (repo_path , _) = gix_discover :: upwards (Path :: new (".")) . ok () ? ; let (gix_dir , work_tree) = repo_path . into_repository_and_work_tree_directories () ; let work_tree = work_tree ? . canonicalize () . ok () ? ; let mut buf = Vec :: with_capacity (512) ; let case = if gix_fs :: Capabilities :: probe (& work_tree) . ignore_case { gix_worktree :: ignore :: glob :: pattern :: Case :: Fold } else { Default :: default () } ; let state = gix_worktree :: stack :: State :: IgnoreStack (gix_worktree :: stack :: state :: Ignore :: new (Default :: default () , gix_worktree :: ignore :: Search :: from_git_dir (& gix_dir , None , & mut buf , gix_worktree :: stack :: state :: ignore :: ParseIgnore { support_precious : false , } ,) . ok () ? , None , gix_worktree :: stack :: state :: ignore :: Source :: WorktreeThenIdMappingIfNotSkipped , Default :: default () ,)) ; Some (gix_worktree :: Stack :: new (work_tree , state , case , buf , Default :: default () ,)) }) () ; Mutex :: new (cache) }) ;
};
}
