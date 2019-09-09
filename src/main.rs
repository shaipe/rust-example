use std::io::{stdout, Write};

use curl::easy::Easy;

// Print a web page onto stdout
fn main() {
    let mut easy = Easy::new();
    easy.url("https://www.baidu.com/").unwrap();
    let html = easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();

    println!("{:?}", html);
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}