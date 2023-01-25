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

/// A buffer to read bits from.
pub struct InBitsBuffer<'a> {
    buffer: [u8; BUFFER_SIZE],
    size: usize,
    // an index into the buffer, to the partial byte being read
    byteposition: usize,
    bitposition: u32, // an index into the current byte
    bitlength: u32,
    input: &'a mut dyn std::io::Read
}

impl<'a> InBitsBuffer<'a> {
    pub fn from(input: &'a mut dyn std::io::Read) -> Self
    {
        let mut ib = InBitsBuffer {
            buffer: [0; BUFFER_SIZE],
            size: 0,
            byteposition: 0,
            bitposition: 0,
            bitlength: u8::BITS,
            input
        };
        ib.populate();
        ib
    }

    fn populate(&mut self)
    {
        self.size = self.input.read(&mut self.buffer).unwrap();
    }

    fn readbits(&mut self) -> Option<usize>
    {
        let mut value: usize = 0;
        let mut bitlength = self.bitlength;
        let mut readbits: u32 = 0;
        while bitlength != 0 {
            if self.size == 0 {
                return None;
            }
            let readable = u8::BITS - self.bitposition;
            let toread = readable.min(bitlength);
            let mask = (1usize << toread) - 1;

            let curbyte = self.buffer[self.byteposition] as usize;
            let curbits = (curbyte >> self.bitposition) & mask;
            value |= curbits << readbits;

            self.bitposition += toread;
            if self.bitposition == u8::BITS {
                self.bitposition = 0;
                self.byteposition += 1;
                if self.byteposition == self.size {
                    self.populate();
                    self.byteposition = 0;
                }
            }

            bitlength -= toread;
            readbits += toread;
        }

        Some(value)
    }

    pub fn read(&mut self) -> Option<usize>
    {
        if self.size == 0 {
            return None;
        }

        let value = self.readbits()?;
        if ESCAPE as usize == value {
            let nextvalue = self.readbits()?;
            if ESCAPE as usize != nextvalue {
                // Single escape: increase window size and try again
                self.bitlength += 1;
                return self.read();
            }
            // Double escape: return the escape value
        }

        Some(value)
    }
}

/// A buffer to write bytes to.
pub struct OutBytesBuffer<'a> {
    buffer: [u8; BUFFER_SIZE],
    position: usize, // an index into the buffer
    output: &'a mut dyn std::io::Write
}

impl<'a> OutBytesBuffer<'a> {
    pub fn from(output: &'a mut dyn std::io::Write) -> Self
    {
        OutBytesBuffer {
            buffer: [0; BUFFER_SIZE],
            position: 0,
            output
        }
    }

    fn writeout(&mut self)
    {
        self.output.write(&self.buffer[0..self.position]).unwrap();
        self.position = 0;
    }

    pub fn flush(&mut self)
    {
        self.writeout();
        self.output.flush().unwrap();
    }

    pub fn write(&mut self, bytes: &[u8])
    {
        let space = self.buffer.len() - self.position;
        if bytes.len() <= space {
            // Enough space for bytes: copy bytes entirely
            self.buffer[self.position..self.position + bytes.len()]
                .copy_from_slice(bytes);
            self.position += bytes.len();
            if self.position == self.buffer.len() {
                self.writeout();
            }
        } else {
            // Not enough space for bytes: copy the prefix
            self.buffer[self.position..].copy_from_slice(&bytes[..space]);
            self.writeout();
            // Try again with the suffix
            self.write(&bytes[space..]);
        }
    }
}
