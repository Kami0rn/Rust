fn main() {
    let x = 5 ;
    let y = 0.5 ;

    let z = y + x as f64 ;

    let msg = String::from("Hello, World!");
    let msg2 = "Hello, World!".to_string();
    let msg3: &str = "Hello, World!";
    let msg4: String = format!("Point: {}, {}",x,y);

    println!("{}" , msg4);
    println!("{}" , msg3);
    println!("{}" , msg2);
    println!("{}" , msg);
    println!("{}" , z);
}
