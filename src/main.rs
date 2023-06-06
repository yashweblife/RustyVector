struct Vector{
    x:u32,
    y:u32
}

impl Vector {
    fn mag(&self)->u32{
        self.x*self.x + self.y*self.y
    }
}