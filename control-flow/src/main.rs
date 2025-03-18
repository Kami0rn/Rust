use std::thread;
use std::time;

fn main() {
    // let weather = "rainy" ;

    // if weather == "sunny" {
    //     println!("Crabby will cross the river by swimming!")
    // } else if weather == "rainy"{
    //     println!("Crabby will build a bridge to stay dry!")
    // } else {
    //     println!("Crabby will wait for better weather!")
    // }

    // -------------------------------------------------///----------------------------------

    // let monster = "dragon" ;

    // match monster {
    //     "goblin" => println!("He use rusty sword to attack!"),
    //     "troll" => println!("Crabby set a trap!"),
    //     "dragon" => println!("Crabby runs for cover"),
    //     _ => println!("Crabby is confused...")
    // }

    // -------------------------------------------------///----------------------------------

    
    let mut wood = 0;

    loop{
        wood += 1;
        println!("Crabby gatherd {} pieces of wood!", wood);

        let ten_millis = time::Duration::from_millis(1000);
        thread::sleep(ten_millis);

        if wood == 10 {
            println!("Crabby finished the boat!");
             break;
        }
    }

}
