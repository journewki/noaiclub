use std::io;


fn main(){
    let mut score = String::new();
    io::stdin()
        .read_line(&mut score)
        .expect("읽기 입력 실패");

    let score = score.trim();
    
    println!("Your score is {score} point.")



}