use std::{fs, io};
use std::fs::read_to_string;
use crate::board::Error::{BoatCount, OutOfBounds, Overlap};



pub struct Board{
    boats: [u8; 4],
    data: [[u8; Board::BSIZE]; Board::BSIZE]
}

pub enum Error {Overlap, OutOfBounds, BoatCount}
pub enum Boat {Vertical(usize), Horizontal(usize)}

impl Board{
    pub const BSIZE: usize = 20;

    pub fn new(boats: [u8;4]) -> Self{
        Self{boats, data: [[b'o'; Self::BSIZE]; Self::BSIZE]}
    }

    pub fn from(s: &str) -> Result<Self,&'static str>{
        let s = read_to_string(s).map_err(|_| "Could not read file.")?;
        let mut s = s.lines();

        let boats: [u8; 4] = (|| {
            let line = s.next()?;
            line.split_whitespace().map(|n| n.parse().ok()).collect::<Option<Vec<_>>>()?.try_into().ok()
        })().ok_or("Bad file format.")?;

        let mut data= [[b' '; Self::BSIZE]; Self::BSIZE];
        for (i, line) in s.take(Self::BSIZE).enumerate() {
            for (j,c) in line.chars().take(Self::BSIZE).enumerate(){
                match c {
                    'o' | 'B' => data[i][j] = c.try_into().unwrap(),
                    _ => Err("Bad file format.")?
                }
            }
        }

        Ok(Board {boats, data})
    }

    pub fn save_to_file(self, path: &str) -> io::Result<()>{
        fs::write(path, self.to_string())
    }

    pub fn add_boat(mut self, boat: Boat, pos: (usize, usize)) -> Result<Self,Error> {

        let mut data = self.data;
        let (r,c)=pos;

        let (dr,dc) = match boat {
            Boat::Horizontal(len) => {
                if self.boats[len - 1] == 0 {Err(BoatCount)?}
                if pos.0 + len > Self::BSIZE {Err(OutOfBounds)?}
                (1,len)
            }
            Boat::Vertical(len) => {
                if self.boats[len-1] == 0 {Err(BoatCount)?}
                if pos.1 + len > Self::BSIZE {Err(OutOfBounds)?}
                (len,1)
            }
        };

        let r_min = r.saturating_sub(1);
        let r_max = (r+dr).min(Self::BSIZE-1);
        let c_min = c.saturating_sub(1);
        let c_max = (c+dc).min(Self::BSIZE-1);

        for x in r_min..=r_max{
            for y in c_min..=c_max{
                if data[x][y] == b'B' {
                    Err(Overlap)?
                }
            }
        }

        match boat {
            Boat::Horizontal(len) => {
                for i in 0..len{
                    data[r][c+i]=b'B';
                }
                self.boats[len-1] -= 1;
            }
            Boat::Vertical(len) => {
                for i in 0..len{
                    data[r+i][c]=b'B';
                }
                self.boats[len-1] -= 1;
            }
        }

        Ok(Board {data, .. self})
    }

    pub fn to_string(&self) -> String {
        let mut out = String::new();

        out.push_str(&(self.boats.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" ")+   "\n"));

        for i in 0..Self::BSIZE{
            for j in 0..Self::BSIZE{
                out.push(self.data[i][j] as char)
            }
            out.push('\n');
        }

        out
    }
}