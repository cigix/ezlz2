pub struct Dictionary {
    entries: Vec<Vec<u8>>
}

impl Dictionary {
    pub fn new_prefilled() -> Dictionary
    {
        let mut entries : Vec<Vec<u8>>
            = Vec::with_capacity((u8::MAX as usize)+ 1);
        for i in u8::MIN..=u8::MAX {
            entries.push(vec![i])
        }
        Dictionary { entries }
    }

    pub fn add(&mut self, bytes: &[u8])
    {
        self.entries.push(Vec::from(bytes))
    }

    pub fn find(&self, bytes: &[u8]) -> Option<usize>
    {
        self.entries.iter()
            .position(|entry| entry == bytes)
    }

    pub fn get(&self, position: usize) -> Option<Vec<u8>>
    {
        self.entries.get(position).cloned()
    }
}
