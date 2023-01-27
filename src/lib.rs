pub mod dict;
pub mod buffers;

const BUFFER_SIZE : usize = 1024;
const ESCAPE : u8 = 0xAA;

/// Get the input and output stream from std::env::args().
pub fn parse_args() -> (Box<dyn std::io::Read>, Box<dyn std::io::Write>)
{
    let argv: Vec<String> = std::env::args().collect();
    let input: Box<dyn std::io::Read> =
        if let Some(filename) = argv.get(1) {
            Box::new(std::fs::File::open(filename).unwrap())
        } else {
            Box::new(std::io::stdin())
        };
    let output: Box<dyn std::io::Write> =
        if let Some(filename) = argv.get(2) {
            Box::new(std::fs::File::create(filename).unwrap())
        } else {
            Box::new(std::io::stdout())
        };
    (input, output)
}
