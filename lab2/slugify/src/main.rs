use std::{env::args, process::exit};
mod table;

fn slugify(s: &str) -> String{

    let mut r = String::new();

    let mut last = ' ';

    for c in s.to_lowercase().chars(){
        let curr = conv(c);
        if curr != '-' || curr != last { r.push(curr); }
        last = curr;
    }

    if last == '-' && r.chars().count() != 1 { r.pop(); }
    
    r
}

fn conv(c: char) -> char {
    
    if c.is_alphanumeric() {
        match table::SUBS_I.chars().position(|x| x == c){
            Some(i) => table::SUBS_O.chars().nth(i).unwrap_or(c),
            None => c,
        }
    } else {
        '-'
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conv_accented() {
        assert_eq!(conv('ì'), 'i');
    }

    #[test]
    fn test_conv_plain() {
        assert_eq!(conv('a'), 'a');
    }

    #[test]
    fn test_conv_invalid() {
        assert_eq!(conv('!'), '-');
    }

    /* 
    #[test]
    fn test_conv_unkwown_accent() {
        
    }
    */

    #[test]
    fn test_slug_multiword() {
        assert_eq!(slugify("ao ao ao"), "ao-ao-ao");
    }

    #[test]
    fn test_slug_accented() {
        assert_eq!(slugify("aò aò aò"), "ao-ao-ao");
    }

    #[test]
    fn test_slug_empty() {
        assert_eq!(slugify(" "), "-");
    }

    #[test]
    fn test_slug_consecuitve_spaces() {
        assert_eq!(slugify("ao  ao  ao"), "ao-ao-ao");
    }

    #[test]
    fn test_slug_consecuitve_invalid() {
        assert_eq!(slugify("!!"), "-");
    }

    #[test]
    fn test_slug_only_invalid() {
        assert_eq!(slugify("?++*"), "-");
    }

    #[test]
    fn test_slug_trailing_space() {
        assert_eq!(slugify("ao "), "ao");
    }

    #[test]
    fn test_slug_trailing_invalid() {
        assert_eq!(slugify("ao ao??"), "ao-ao");
    }

    #[test]
    fn test_slug_uppercase() {
        assert_eq!(slugify("AO AO AO"), "ao-ao-ao");
    }
}



fn main() {

    let mut args = args().skip(1);

    let s: String = match args.next(){
        None => {
            eprintln!("Specify a file to check");
            exit(1)
        },
        Some(s) => s
    };

    println!("slug : {}", slugify(&s))

}
