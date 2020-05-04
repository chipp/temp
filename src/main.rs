use temp_reader::*;

fn main() {
    let mut args = std::env::args().skip(1);

    match args.next().as_ref().map(String::as_str) {
        Some("measure") => measure(),
        Some("list") => list(),
        Some("info") => info(),
        Some(command) => panic!("invalid command {}", command),
        None => panic!("usage: temp_reader (measure|list|info)"),
    }
}
