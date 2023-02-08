use cmd_lib::run_cmd;

pub fn run_command(absolute_path_to_forked_project: String, upstream_addr: String, branch_name: String) {
    // upstreamAddr example: git@github.com:mianwu515/rust-world-spr23.git
    // branchName: main/master
   assert!(run_cmd!(cd "/Users/wumian/Documents/Spring2023Courses/ECE651SoftwareEngineering/gitlab/ece651-spr23").is_ok());
   run_cmd!(git remote add upstream "git@github.com:mianwu515/rust-world-spr23.git");
   assert!(run_cmd!{
        git fetch upstream;
        git checkout master;
        git pull upstream master;
        git push origin master;
    }.is_ok());
}

