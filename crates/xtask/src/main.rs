use std::{error::Error, path::PathBuf};
// Change the alias to `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn Error>>;

mod flags {
    use std::path::PathBuf;

    xflags::xflags! {

    cmd repo {
        /// EX: `cargo xtask set-version node/package.json '0.0.2'`
        cmd set-version {
            /// path to file with version key
            required path: PathBuf
            /// how is version spelled and delimited, without whitespace
            /// ex. "version:"
            optional --term search_term: String
            optional --prefix prefix: String
            optional --postfix postfix: String
            optional --add-branch
            optional --dry-run
        }
    }
    }
}

fn main() {
    match flags::Repo::from_env() {
        Ok(repo) => match repo.subcommand {
            flags::RepoCmd::SetVersion(flags) => {
                // move_folder_contents_to_parent_and_delete_all_else(
                //     flags.path,
                //     flags.msg,
                //     flags.version,
                // )
                // .unwrap();
            }
        },
        Err(err) => err.exit(),
    }
}

#[allow(dead_code)]
fn cargo() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())
}

fn project_root() -> PathBuf {
    std::path::Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
