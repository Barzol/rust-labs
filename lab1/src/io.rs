use std::io::{BufRead,BufReader, Seek};
use std::fs::File;
//use std::io::{Seek, SeekFrom};

pub fn read_file(filename : String, head : usize) -> Result<(), String> {
    let mut file = File::open(filename).expect("File not found");

    // lines iterator, lines() returns a Lines that has the method count 
    // for counting lines, it consumes the iterator so we have to do a ref

    let temp_iter = BufReader::new(&file).lines();
    println!("rows : {}", temp_iter.count());

    match file.rewind() {
        Ok(_) => (),
        Err(_) => println!("Rewind failed"),
    }

    let lines_iter = BufReader::new(&file).lines();

    let mut i = 0;

    println!("head ({head}) :");
    for line in lines_iter{
        if i < head {
            println!("{}", line.expect("not found"));
            i +=1;
        } else {
            i +=1;
        }
    }

    // prints the number of lines
    let n_lines = i;
    println!("The file has {} lines", n_lines );

    Ok(())

}


