use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    #[structopt(name = "addblock")]
    /// Adds a block to the blockchain (use `addblock --help` for more info)
    AddBlock {
        #[structopt(short, long)]
        /// adds a block to the blockchain
        data: String,
    },

    #[structopt(name = "printchain")]
    /// Prints all the blocks of the blockchain
    PrintChain,
}

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(subcommand)]
    pub command: Command,
}
