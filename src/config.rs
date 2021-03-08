use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Config {
    /// Convert lowercase characters to phonetics as well
    #[structopt(short, long)]
    pub lowercase: bool,
}
