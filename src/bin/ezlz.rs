use ezlz2::buffers;
use ezlz2::dict;

fn main() {
    let (mut input, mut output) = ezlz2::parse_args();

    let mut ib = buffers::InBytesBuffer::from(&mut input);
    let mut ob = buffers::OutBitsBuffer::from(&mut output);
    let mut dict = dict::Dictionary::new_prefilled();
    let mut work: Vec<u8> = Vec::new();

    while let Some(byte) = ib.read() {
        work.push(byte);

        for size in 2..=work.len() {
            let slice = work.get(0..size).unwrap();
            if let None = dict.find(slice) {
                // Largest pattern that exists in the dict
                let prevslice = &slice[..size - 1];
                let code = dict.find(prevslice).unwrap();
                ob.write(code);
                dict.add(slice);
                work.drain(0..size - 1);
                break;
            }
        }
    }

    while ! work.is_empty() {
        for size in (1..=work.len()).rev() {
            let slice = work.get(0..size).unwrap();
            if let Some(code) = dict.find(slice) {
                ob.write(code);
                work.drain(0..size);
                break;
            }
        }
    }
    ob.flush();
}
