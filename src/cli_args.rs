use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct CLIArgs {
    #[arg(help="the source file you want to variable-ize")]
    pub(crate) input_file: String
}
