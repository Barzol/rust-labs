// paste this file into main.rs

use std::fs::{self};
use std::env::args;
use std::process::exit;

fn stats(text: &str) -> [u32; 26] {

    let mut res: [u32; 26] = [0; 26];

    for c in text.to_ascii_lowercase().chars().filter( |c| c.is_alphabetic() ) {
        
        res[ (c as u8 - 'a' as u8) as usize ] += 1;

    }

    return res;

}

fn is_pangram(counts: &[u32]) -> bool {
    if counts.len() != 26 {return false;}
    counts.iter().all(|c| *c >= 1)
}

// call this function from main
// load here the contents of the file
pub fn run_pangram() {

    let mut args = args();

    if args.len() != 1 { eprintln!("Error")}

    let file: String = match args.next(){
        None => {
            eprintln!("Usage : specify a file to check");
            exit(1)
        },
        Some(path) => {
            fs::read_to_string(path).expect("File not Found")
        }
    };

    let r = stats(&file);

    if is_pangram(&r) {
        println!("{} is a pangram", file);

        for (i, s) in r.iter().enumerate() {
            let letter = (b'a' + i as u8) as char;
            println!( "{letter}\t{s}" )
        }

    } else {
        println!("{} is not a pangram", file);
    }
    



    

}


// please note, code has been splitted in simple functions in order to make testing easier

#[cfg(test)] // this is a test module
mod tests
{   
    // tests are separated modules, yuou must import the code you are testing
    use super::*;
    
    #[test]
    fn test_all_ones() {
        let counts = [1; 26];
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_some_zeros() {
        let mut counts = [0; 26];
        counts[0] = 1;
        counts[1] = 1;
        assert!(!is_pangram(&counts));
    }
    
    #[test]
    fn test_increasing_counts() {
        let mut counts = [0; 26];
        for i in 0..26 {
            counts[i] = i as u32 + 1;
        }
        assert!(is_pangram(&counts));
    }

    #[test]
    fn test_wrong_size()  {
        let counts = [1; 25];
        assert!(!is_pangram(&counts));
    }    
    
    #[test]
    fn test_stats_on_full_alphabet() {
        let counts = stats("abcdefghijklmnopqrstuvwxyz");
        for c in counts {
            assert!(c == 1);
        }
    }

    #[test]
    fn test_stats_on_empty_string() {
        let counts = stats("");
        for c in counts {
            assert!(c == 0);
        }
    }

    #[test]
    fn test_stats_missing_char() {
        let counts = stats("abcdefghijklmnopqrstuvwxy");
        for c in counts.iter().take(25) {
            assert!(*c == 1);
        }
        assert!(counts[25] == 0);

    }

    #[test]
    fn test_stats_on_full_tring() {
        let contents = "The quick brown fox jumps over the lazy dog";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test]
    fn test_stats_with_punctuation() {
        let contents = "The quick brown fox jumps over the lazy dog!";
        let counts = stats(contents);
        for c in counts {
            assert!(c > 0);
        }
    }

    #[test] 
    fn test_missing_char_on_full_string() {
        let contents = "The quick brown fox jumps over the laz* dog";
        let counts = stats(contents);
        println!("{:?}", counts);
        for (i, c) in counts.iter().enumerate() {
            if i == 24 {
                assert!(*c == 0);
            } else {
                assert!(*c > 0);
            }
            
        }
    }

    #[test]
    fn test_is_pangram() {
        let counts = stats("The quick brown fox jumps over the lazy dog");
        assert!(is_pangram(&counts));
    }
}

//fn main() {
//    run_pangram();
//}

