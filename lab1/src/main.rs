mod io;
mod cli;

use std::process;

/**
 * Commands :
 * cargo run -- data.csv
 * 
 * cargo run -- data.csv --head 5
 */


fn main() {

    let _ = match cli::pars_args() {
        Ok(data) => 
            io::read_file(data.0, data.1 as usize).map_err(|e| {
                eprintln!("Error in I/O: {}",e);
                process::exit(1);
            }),
        Err(e) => {
            eprintln!("{}",e);
            process::exit(1);
        }
    };
    
}
