use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
pub struct UiConfig {
    /// Activate debug mode
    #[structopt(short = "b", long = "beats", default_value = "./beat.medaddy")]
    pub beat: PathBuf
}


