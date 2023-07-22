use std::path::PathBuf;

use argh::FromArgs;

#[derive(FromArgs)]
/// Please, report issues to <https://github.com/veeso-dev/tokyo-draft>
pub struct Args {
    /// file where to write PID to
    #[argh(option)]
    pub pidfile: Option<PathBuf>,
}
