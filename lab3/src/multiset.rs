pub struct MultiSet {
    n       : u32,
    counter : u32,
    first   : u32,
    to_gen  : u32,
}

impl MultiSet {
    pub fn new(n: u32) -> MultiSet {
        MultiSet { 
            n,
            counter : 0,  
            first   : (n * (n - 1)) / 2,
            to_gen  : n,
        }
    }
}

impl Iterator for MultiSet {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.to_gen == 0 {
            None
        } else {
            self.to_gen -= 1;
            self.counter = (self.counter % self.n) + 1;
            Some(self.first + self.counter)
        }
    }
}