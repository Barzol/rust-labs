use crate::table;

pub trait MySlug{
    fn is_slug(&self) -> bool;
    fn to_slug(&self) -> String;
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

// impl for bot String and str
impl <T: AsRef<str>> MySlug for T {

    fn is_slug(&self) -> bool {

        return self.as_ref()
            .chars()
            .all(|c| 
                c.is_alphanumeric() || c == '-' );

    }

    fn to_slug(&self) -> String{

        let mut r = String::new();

        let mut last = ' ';

        // convert self into a string ref
        for c in self.as_ref().to_lowercase().chars(){
            let curr = conv(c);
            if curr != '-' || curr != last { r.push(curr); }
            last = curr;
        }

        if last == '-' && r.chars().count() != 1 { r.pop(); }
        
        r
    }

}

