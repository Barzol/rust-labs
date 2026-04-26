mod table;
mod slugify;
use crate::slugify::MySlug;

fn main() {

    let s1 = String::from("Hello String");
    let s2 = "hello-slice";

    println!("{}", s1.is_slug());
    println!("{}", s2.is_slug());

    let s3: String = s1.to_slug();
    let s4: String = s2.to_slug();
    println!("s3:{}, \ns4:{}", s3, s4);

}
