use std::io;

fn main(){
    let mut yas = String::new();
    io::stdin()
        .read_line(&mut yas)
        .expect("문자를 입력하세요");

    let yas:i32 = yas.trim().parse().expect("yas");
    println!("{}", yas+2)

    
    
    
}