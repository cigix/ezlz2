use ezlz2::buffers;
use ezlz2::dict;

fn main() {
    let (mut input, mut output) = ezlz2::parse_args();

    let mut ib = buffers::InBitsBuffer::from(&mut input);
    let mut ob = buffers::OutBytesBuffer::from(&mut output);
    let mut dict = dict::DecompressionDictionary::new_prefilled();
    let mut work: Vec<u8> = Vec::new();

    while let Some(value) = ib.read() {
        if let Some(entry) = dict.get(value) {
           if ! work.is_empty() {
               work.push(*entry.get(0).unwrap());
               dict.add(&work);
           }
           work = entry.clone();
        } else {
            work.push(*work.last().unwrap());
            dict.add(&work);
        }
        ob.write(&work);
    }

    ob.flush();
}
