use cmd_lib::run_cmd;

pub fn run_command(
    absolute_path_to_forked_project: String,
    upstream_addr: String,
    branch_name: String,
) {
    // upstreamAddr example: git@github.com:mianwu515/rust-world-spr23.git
    // branchName: main/master
    assert!(run_cmd!(cd "$absolute_path_to_forked_project").is_ok());
    run_cmd!(git remote add upstream "$upstream_addr"); // remote upstream may have already existed. No assertion here
    assert!(run_cmd! {
        git fetch upstream;
        git checkout "$branch_name";
        git pull upstream "$branch_name";
        git push origin "$branch_name";
    }
    .is_ok());
}
