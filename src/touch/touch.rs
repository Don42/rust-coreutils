use std::env;
use std::fs::File;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut f = match File::create(&args[1]) {
    Err(e) => {
        println!("Couldn't open foo.txt {}", e);
        return;
    },
    Ok(f) => f,
};
    println!("{:?}", f.write_all(b""));
}
