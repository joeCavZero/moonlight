use moonlight::moonlight;

pub fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        1 => {
            println!("Please provide a file path as an argument.");
            return;
        }
        2 => {
            match args[1].as_str() {
                "--version" | "-v" => {
                    println!("Moonlight version 0.1.0");
                }
                _ => {
                    let mut ml = moonlight::Moonlight::new();
                    ml.run(&args[1]);
                }
            }
        }
        _ => {
            println!("Too many arguments provided. Please provide only one file path.");
        }
    }
}