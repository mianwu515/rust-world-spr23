# A tool to sync between upstream, forked origin and local.

## Precondition

- You have
    - a remote origin that was forked from an upstream source
    - a local repo that was cloned from your remote origin.

![upstream-origin-local](https://premaseem.files.wordpress.com/2016/01/githubfork.png?w=640)

## Usage
- Now suppose your local repo path is `PATH`, your remote branch is `main` (in gitlab, it's `master`), and the upstream path is `UPSTREAM`
- You could then run this program with cmd right in current directory:
```{bash}
cargo run -- syncfork --path PATH -- UPSTREAM main
```
- For example, your are in your local directory "~/rust-world-spr23", (and you have added your remote origin "git@github.com:[your github id here]/rust-world-spr23"), and the upstream is "https://github.com/mianwu515/rust-world-spr23.git" (HTTPS url version, or you can use SSH url version here, which might be "git@github.com:mianwu515/rust-world-spr23.git")
_ Then the cmd would be
```
cargo run -- syncfork --path "~/rust-world-spr23" -- "git@github.com:mianwu515/rust-world-spr23.git" main
```
_ Done! Your local and remote origin have been updated with the upstream!

