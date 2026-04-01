use std::fs;

const BSIZE: usize = 20;

pub struct Board{
    boats: [u8; 4],
    data: [[u8; BSIZE]; BSIZE],
}

pub enum Error { Overlap, OutOfBounds, BoatCount }

pub enum Boat { Vertical(usize), Horizontal(usize) }

impl Board {

    /// Creates an empty board with boats vector 
    pub fn new(boats: &[u8]) -> Board {

        let board = Board {
            /* 
                boats[..4] creates a slice of first 4 elements
                try_into() converts the slice in an array of 4 elements
                unwrap_or() if the slice is shorter than 4, put default [0,4]
            */ 
            boats : boats[..4].try_into().unwrap_or([0;4]),
            data : [[b'.'; BSIZE]; BSIZE]
        };

        fs::write("board.txt", board.to_string()).expect("Error in writing");

        board
        
    }

    /// Creates a Board from the file contents
    pub fn from(s: String) -> Board {

        let r_board = fs::read("board.txt");


    }

    /// Adds a boat : returns the new boat or an error
    pub fn add_boat(self, boat: Boat, pos : (usize, usize)) -> Result<Board,Error> {
        
        let board = fs::read_to_string("board.txt");

        
    }

    /// Converts the board into a string that can be saved on a file
    pub fn to_string(&self) -> String {
        todo!()
    }

}