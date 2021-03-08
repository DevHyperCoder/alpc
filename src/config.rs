use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Config {
    /// Convert lowercase characters to phonetics as well
    #[structopt(short, long)]
    pub lowercase: bool,

    /// Add verbosity to the output (not machine readable)
    #[structopt(short, long)]
    pub verbose: bool,
}
