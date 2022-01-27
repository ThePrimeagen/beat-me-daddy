use structopt::StructOpt;

#[derive(Debug, StructOpt, Clone)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
pub struct PiOpts {
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    pub debug: bool,

    /// If the program runs in server mode
    #[structopt(short = "s", long = "server")]
    pub server: bool,

    ///
    #[structopt(short = "p", long = "port", default_value = "6969")]
    pub port: u16,

    #[structopt(short = "a", long = "addr")]
    pub addr: String,

    #[structopt(short = "c", long = "command", default_value = "sonic_pi")]
    pub command: String,


    #[structopt(short = "b", long = "bpm", default_value = "120")]
    pub bpm: u16,
}
