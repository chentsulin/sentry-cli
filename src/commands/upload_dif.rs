// upload-dif is a backwards compatible, hidden alias for `debug-files upload`.

use anyhow::Result;
use clap::{ArgMatches, Command};

pub fn make_command(command: Command) -> Command {
    crate::commands::debug_files_upload::make_command(command).hide(true)
}

pub fn execute(matches: &ArgMatches) -> Result<()> {
    crate::commands::debug_files_upload::execute(matches)
}
