pub mod solution {
    use crate::solution::CBufError::Full;
    pub struct CircularBuffer <T> {
        pub buf : Vec<Option<T>>,
        pub head : usize,   // index for read data
        pub tail : usize,   // index for insert data
        pub size : usize,   // current number of elements
    }

    #[derive(Debug)]
    pub enum CBufError {
        Full
    }

    impl <T> CircularBuffer<T> {

        fn is_full(&self) -> bool {self.size == self.buf.len()}
        
        pub fn new(capacity: usize) -> Self {

            CircularBuffer { 
                // declare buf as an empty vector with capacity, then
                // use map for create an iterator that sets each item in
                // the vector as None, and then collect for convert the iterator
                // into a collection
                buf : (0..capacity)
                    .map(|_| None)
                    .collect(),
                head: 0, 
                tail: 0, 
                size: 0, 
            }
        }

        pub fn write(&mut self, item: T) -> Result<(), CBufError> {
            // check if the vector is full
            if self.is_full() {
                Err(Full)?
            }
            
            // at index 'tail' inserts an item in the Some of 
            // the Option of the vector
            self.buf[self.tail] = Some(item);
            // increase index tail
            self.tail = (self.tail + 1) % self.buf.len();
            // count of elements
            self.size += 1;

            Ok(())
        }

        pub fn read(&mut self) -> Option<T> {
            if self.size == 0 { return None; }
            // at index head reads an item in the Some of the
            // Option, leaving None at its place
            let item = self.buf[self.head].take();
            // increase index tail
            self.head = (self.head + 1) % self.buf.len();
            // count of elements
            self.size -= 1;

            item
        }

        pub fn clear (&mut self) { 
            self.buf = self.buf.iter().map(|_| None).collect();
            self.head = 0;
            self.tail = 0;
            self.size = 0;
        }

        pub fn size (&self ) -> usize { 
            self.size 
        }

        // Scrive forzando la sovrascrittura dell ’ elemento piu ’ vecchio
        pub fn overwrite (&mut self , item : T ) { 
            if  self.is_full(){ self.read(); }
            let _ = self.write(item);
        }

        // Riorganizza il buffer rendendolo contiguo in memoria
        pub fn make_contiguous (&mut self) { 
            if self.head != 0 {
                self.buf.rotate_left(self.head);
                // update head
                self.head = 0;
                // tail must be at the last element 
                self.tail = self.size;
            }
        }
    }

    
}

