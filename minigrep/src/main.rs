use std::{env, process};
use minigrep::Config;
fn main() {
    //let args :Vec<String> = env::args().collect();
    let config = Config::build(env::args()).unwrap_or_else(|err|{
     eprintln!("Problem parsing arguments : {err}"); //错误会写入到控制台    cargo run > output.txt
     process::exit(1)
   });
    //println!("{:?}",env::current_dir()); //获取当前目录
   /*  let contents = fs::read_to_string(config.file_path)
    .expect_err("Shoud have been able to read the file");
    println!("With text:\n{contents}");
    */
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    if let Err(e) = minigrep::run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
/*cmd IGNORE_CASE=1 cargo run -- frog sample.txt   cargo run -- frog sample.txt > output.txt*/
/* fn run(config:Config){
    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read file");
    println!("With text:\n{contents}");
}
 */

/* fn parse_config(args:&[String]) -> Config{
    let query =args[1].clone();
    let file_path = args[2].clone();
    Config { query, file_path }
} */

