use std::env::Args;
use std::str::FromStr;
mod cli;
use crate::cli::run_cli;

use crate::board::{Board, Boat, Error};

pub mod board;

fn main() {
    let cli = |args: Args, cmdname| -> Result<(),String> {
        let need_help = format!(
            "Usage:\n
                 - {cmdname} <FILE> new <c1,c2,c3,c4>\n
                 - {cmdname} <FILE> add_boat H|V,<LEN>,<X>,<Y>\n
                "
        );

        let args: Vec<String> = args.collect();

        if args.len() != 3 { Err(&need_help)? }

        let file = &args[0];
        let subcmd = &args[1];

        /*
         * la funzione trasforma una lista di stringhe in una lista di un altro tipo
         * 
         * - iter() : creates an iterator over the strings
         * - .map() : tenta di convertire la stringa con
         *      - parse() : restituisce un result
         *      - ok() : trasfroma Result in Option
         * - collect() : trasforma un iteratore di Option<T> direttamente in
         *               un Option<Vec<T>>, se anche solo uno degli Option è un
         *               None allora restituisce tutto None
         */

        fn parse_nums<T:FromStr>(s_list: Vec<&str>) -> Option<Vec<T>> {
            s_list
                .iter()
                .map(|s| 
                    s.parse().ok()
                )
                .collect()
        }


        match subcmd.as_str() {
            "new" => {
                /*
                 * args[2].split(',').collect::<Vec<_>>() : prende una stringa, la divide alle virgole creando
                 *                                          un iteratore e crea un Vec<&str>
                 * 
                 * parse_nums() : restituisce un option, in questo caso Option<Vec<&str>>
                 * 
                 * and_then( |v| v.try_into().ok() ) : se il risultato precedente è un Some, tenta di convertire
                 *                                     il vec in un array [u8; 4], se dovesse fallire allora
                 *                                     ok() lo trasforma in un None
                 * 
                 * .ok_or_else( || format!("Invalid arguments: {}", args[2]))? : 
                 *      se è un None, allora genera con format! un messaggio d'errore, altrimenti trasformi  
                 *      l'Option in un Result. Successivamente con '?' se è un Ok allora assegna il valore a
                 *      boats, altrimenti se è un Err interrompe la funzione 
                 * 
                 */
                let boats: [u8; 4] = parse_nums(args[2]
                    .split(',')
                    .collect::<Vec<_>>()
                )
                .and_then( |v| v.try_into().ok() )
                .ok_or_else( || format!("Invalid arguments: {}", args[2]))?;

                Board::new(boats).save_to_file(file).map_err(|_| "Invalid file.")?;

            }
            "add_boat" => {

                // creo iteratore sulla stringa, dividendo per virgole
                let mut params = args[2].split(',');
                // estrae il primo elemento e lo mette dentro direction, ora params è solo numeri
                let direction = params.next().unwrap();
                /*
                 *  params.collect() : prendo l'iteratore e lo converte in un Vec<&str>
                 *  parse_nums : restituisce un Option
                 *  
                 *  and_then(|v| v.try_into().ok()) : 
                 *      se restituisce un Some, allora converte il 
                 *      Vec<&str> in Vec<usize> rispettivamente [len,x,y]
                 * 
                 *  ok_or_else( || format!("Invalid arguments: {}", args[2]))? :
                 *      Stessa cosa di prima
                 */
                let [len,x,y]: [usize;3] = parse_nums(params.collect())
                    .and_then(|v| v.try_into().ok())
                    .ok_or_else(|| format!("Invalid arguments: {}.", args[2]))?;

                // se len non è tra 1 e 4 allora lancia errore
                if !(1..=4usize).contains(&len){ Err("Boat length must be in range [1,4]")? }

                // carica la board attuale
                let board = Board::from(file).map_err(|_| "Invalid file.")?;

                // match per mettere in boat la direzione
                let boat = match direction {
                    "H" => Boat::Horizontal(len),
                    "V" => Boat::Vertical(len),
                    s => Err(format!("Error parsing '{}': should be 'H' or 'V'.", s))?
                };

                /*
                 * aggiungo la boat, map_err creo una closure con un match per gestire i diversi
                 * casi di errore, infine restituisco l'errore alla funzione principale.
                 * Poi save_to_file salva il file con l'aggiunta, map_err per gestire l'errore 
                 */ 
                board.add_boat(boat, (x,y)).map_err(|e| match e {
                    Error::BoatCount => format!("Could not add boat: not enough boats of size {len}"),
                    Error::Overlap => "Could not add boat: other boats overlap".into(),
                    Error::OutOfBounds => format!("Could not add boat: not enough boats size {len}")
                })?.save_to_file(file).map_err(|_| "Error saving to file.")?


            }

            _ => Err(&need_help)?
        }

        Ok(())
    };

    run_cli(cli);
}