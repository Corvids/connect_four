use std::io;

fn main() {
    welcome_message();

    let mut board = [[0u8; 6]; 7];

    assign_players();
}

fn welcome_message() {
    println!("Starting a new game!");
}

// assign player to a color;
// computer will be other color
fn assign_players() -> (String, String) {
    loop {
        let mut player_choice = String::new();

        println!("Choose a color - red/yellow: ");
        
        io::stdin().read_line(&mut player_choice)
            .ok()
            .expect("Failed to read line");

        if check_color_input(&player_choice) == false {
            println!("Please enter 'red' or 'yellow' >:(");
        } else {
            println!("Player has successfully chosen a color; implemenet pls");
            return ("".to_string(), "".to_string());
        }
    }
}

fn check_color_input(choice: &str) -> bool {
    if choice.trim().to_lowercase() == "yellow" || choice.trim().to_lowercase() == "red" {
        return true;
    } else {
        return false;
    }
}

fn print_board() {
    
}

fn update_board() {
    
}

fn check_if_winner() -> bool {
    return false;
}

fn check_playable() -> bool {
    return true;
}
