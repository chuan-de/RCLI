mod process;
mod cli;
mod utils;

pub use cli::{Opts, Subcommand, Base64SubCommand, TextSubCommand,TextSignFormat};
pub use utils::{get_reader};
pub use process::*;

