mod game;

fn main() {
    println!("BACKJACK GAME");
    println!("Are you ready to play?");
    println!();

    
    loop {
        let mut answer = String::new();
        let c: char;
        println!("Type yes or no");

        std::io::stdin().read_line(&mut answer).unwrap();
        c = answer.chars().nth(0).unwrap();

        if c == 'n' || c == 'N' {
            println!("Bye!");
            break; 
        }
        
        if c == 'Y' || c == 'y' {
            game::play();
            println!("Do you want to keep playing?");
        }
    }
}
