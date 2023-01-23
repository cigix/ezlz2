use crate::BUFFER_SIZE;

/// A buffer to read bytes from.
pub struct InBytesBuffer<'a> {
    buffer: [u8; BUFFER_SIZE],
    size: usize,
    position: usize, // an index into the buffer
    input: &'a mut dyn std::io::Read
}

impl<'a> InBytesBuffer<'a> {
    pub fn from(input: &'a mut dyn std::io::Read) -> Self
    {
        let mut ib = InBytesBuffer {
            buffer: [0; BUFFER_SIZE],
            size: 0,
            position: 0,
            input
        };
        ib.populate();
        ib
    }

    fn populate(&mut self)
    {
        self.size = self.input.read(&mut self.buffer).unwrap();
    }

    pub fn read(&mut self) -> Option<u8>
    {
        if self.size == 0 {
            return None;
        }

        let value = *self.buffer.get(self.position).unwrap();

        self.position += 1;
        if self.position == self.size {
            self.populate();
            self.position = 0;
        }

        Some(value)
    }
}
