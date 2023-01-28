use cmd_lib::run_cmd;

pub fn run_command(upstream_addr: String, branch_name: String) {
    // upstreamAddr example: git@github.com:mianwu515/rust-world-spr23.git
    // branchName: main/master
    assert!(run_cmd!{
        cd /workspaces/rust-world-spr23;// change this directory as needed
        pwd;
        git remote add upstream $[upstreamAddr];
        git fetch upstream;
        git checkout $[branchName];
        git pull upstream $[branchName];
        git push origin $[branchName];
        pwd;
    }.is_ok());
        /*&& // cd into the parent directory (project root); if you are in your root, remove this line of cmd
        git remote add upstream $[upstreamAddr]
        || true && git fetch upstream 
        && git checkout $[branchName] 
        && git pull upstream $[branchName] 
        && git push origin $[branchName])?;*/
}

