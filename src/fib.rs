pub struct Fibonacci {
    x: i32,
    y: i32, 
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci {x: 1, y: 1}
    }
}

impl Iterator for Fibonacci {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let z = self.y + self.x;
        self.x = self.y;
        self.y = z;
        Some(self.x)
    }
}