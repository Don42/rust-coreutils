use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    for argument in args.iter() {
        print!("{} ", argument);
    }
    print!("\n");
}
