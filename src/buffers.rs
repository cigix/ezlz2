use crate::BUFFER_SIZE;
use crate::ESCAPE;

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

/// A buffer to write bits to.
pub struct OutBitsBuffer<'a> {
    buffer: [u8; BUFFER_SIZE],
    // an index into the buffer, to the partial byte being written
    byteposition: usize,
    bitposition: u32, // an index into the current byte
    bitlength: u32,
    output: &'a mut dyn std::io::Write
}

impl<'a> OutBitsBuffer<'a> {
    pub fn from(output: &'a mut dyn std::io::Write) -> Self
    {
        OutBitsBuffer {
            buffer: [0; BUFFER_SIZE],
            byteposition: 0,
            bitposition: 0,
            bitlength: u8::BITS,
            output
        }
    }

    fn writeout(&mut self)
    {
        self.output.write(&self.buffer[0..self.byteposition]).unwrap();
        self.byteposition = 0;
    }

    pub fn flush(&mut self)
    {
        if self.bitposition != 0 {
            self.byteposition += 1;
        }
        self.writeout();
        self.output.flush().unwrap();
    }

    fn writebits(&mut self, mut value: usize)
    {
        let mut bitlength = self.bitlength;
        while bitlength != 0 {
            self.buffer[self.byteposition] |= (value as u8) << self.bitposition;

            let writeable = u8::BITS - self.bitposition;
            let written = writeable.min(bitlength);

            self.bitposition += written;

            if self.bitposition == u8::BITS {
                self.bitposition = 0;
                self.byteposition += 1;
                if self.byteposition == self.buffer.len() {
                    self.writeout();
                }
            }

            value >>= written;
            bitlength -= written;
        }
    }

    pub fn write(&mut self, value: usize)
    {
        while (1 << self.bitlength) <= value {
            // Expand the window
            self.writebits(ESCAPE as usize);
            self.writebits((!ESCAPE) as usize);
            self.bitlength += 1;
        }
        if ESCAPE as usize == value {
            // Write it twice
            self.writebits(value);
        }
        self.writebits(value);
    }
}
