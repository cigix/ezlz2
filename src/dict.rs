use std::collections::HashMap;

pub struct CompressionDictionary {
    counter: usize,
    entries_by_size_minus_one: Vec<HashMap<Vec<u8>, usize>>
}

impl CompressionDictionary {
    pub fn new_prefilled() -> CompressionDictionary
    {
        let mut map_size_one = HashMap::new();
        for i in u8::MIN..=u8::MAX {
            map_size_one.insert(vec![i], i as usize);
        }
        CompressionDictionary {
            counter: u8::MAX as usize + 1,
            entries_by_size_minus_one: vec![map_size_one]
        }
    }

    pub fn add(&mut self, bytes: &[u8])
    {
        while self.entries_by_size_minus_one.len() < bytes.len() {
            self.entries_by_size_minus_one.push(HashMap::new());
        }

        self.entries_by_size_minus_one
            .get_mut(bytes.len() - 1)
            .unwrap()
            .insert(Vec::from(bytes), self.counter);
        self.counter += 1;
    }

    pub fn find(&self, bytes: &[u8]) -> Option<usize>
    {
        self.entries_by_size_minus_one
            .get(bytes.len() - 1)
            ?
            .get(bytes)
            .copied()
    }
}

pub struct DecompressionDictionary {
    entries: Vec<Vec<u8>>
}

impl DecompressionDictionary {
    pub fn new_prefilled() -> DecompressionDictionary
    {
        let mut entries : Vec<Vec<u8>>
            = Vec::with_capacity((u8::MAX as usize)+ 1);
        for i in u8::MIN..=u8::MAX {
            entries.push(vec![i])
        }
        DecompressionDictionary { entries }
    }

    pub fn add(&mut self, bytes: &[u8])
    {
        self.entries.push(Vec::from(bytes))
    }

    pub fn get(&self, position: usize) -> Option<Vec<u8>>
    {
        self.entries.get(position).cloned()
    }
}
