macro_rules! deps {
    () => {
        CargoWorkspaceInfo!();
        NixFlakeInfo!();
        SubmoduleInfo!();
    };
}

macro_rules! RepoState {
    () => {
        deps!();
        # [derive (Debug , Default , PartialEq , Eq , Clone , Hash)] # [cfg_attr (feature = "serde_enabled" , derive (serde :: Serialize , serde :: Deserialize))] pub struct RepoState { pub repo_url : String , pub owner : String , pub repo_name : String , pub target_org : String , pub target_branch : String , pub submodules : Vec < SubmoduleInfo > , pub cargo_workspaces : Vec < CargoWorkspaceInfo > , pub nix_flakes : Vec < NixFlakeInfo > , }
    };
}

RepoState!();