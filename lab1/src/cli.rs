use std::{env::args};

pub fn pars_args() -> Result<(String, u32), String> {
    // skip 1 because the first element returned by args is the path of the executable
    let mut arg = args().skip(1);

    let file = arg.next()
        .ok_or_else(|| "Not enough parameters : type file name".to_string())?;

    let flag = match arg.next(){

        // if none, default value is 10
        None => 10,

        // if there is something, check if it is "--head"
        Some(s) if s == "--head" => 
            // check if the parameter of --head is right or wrong
            arg.next()
            .ok_or("Missing value for --head".to_string())
            .and_then(|n| 
                    n.parse::<u32>()
                        .map_err(|_| format!("Invalid argument : {}",n))
            )?,
        // if there is something that is not "--head"
        Some(o) => return Err(format!("Invalid Argument: {}", o)),

    };

    Ok((file, flag))

    
}