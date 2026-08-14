use std::io;

fn main(){
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("not a word");
    
    let words:Vec<&str> = input.trim().split_whitespace().collect();

    let a:i32 = words[0].parse().unwrap();
    let b:i32 = words[1].parse().unwrap();
    
    println!("{} {} {}", a,b,a+b);
}