mod cli_args;
mod errors;

use clap::Parser;
use cli_args::CLIArgs;
use std::{io, process, fs, env};
use errors::TovarusError;
use shell_escape::escape;


fn set_env_var(key: &str, val: &str, overwrite: bool) -> Result<(), TovarusError> {
    if !overwrite {
        if let Ok(_) = env::var(key) {
            return Err(TovarusError::EnvVarAlreadyExists)
        }
    }
    env::set_var(key, val);
    Ok(())
}


fn read_csv(filename: String) -> Result<(), TovarusError> {
    let mut file = match fs::File::open(&filename) {
        Ok(f) => f,
        Err(e) => return Err(TovarusError::General(e.to_string()))
    };
    let mut reader = csv::Reader::from_reader(file);
    for (row_idx,row_result) in reader.records().enumerate() {
        let row = match row_result {
            Ok(r) => r,
            Err(e) => return Err(TovarusError::General(e.to_string()))
        };
        let num_cols = row.len().to_string().len();
        for (col_idx, column) in row.iter().enumerate() {
            //println!("R{:0row_width$}_C{:0col_width$}={column}", row_idx, col_idx, row_width=4, col_width=num_cols);
            println!("export R{row_idx}_C{col_idx}={}",escape(column.into()));
        }
    }
    Ok(())
}

fn main() {
    let cli = CLIArgs::parse();
    match read_csv(cli.input_file) {
        Ok(_) => {},
        Err(e) => {
            println!("{e}");
        }
    };
}
