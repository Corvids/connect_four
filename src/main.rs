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
            return assign_colors(player_choice);
        }
    }
}

fn assign_colors(player_color: String) -> (String, String) {
    if player_color.trim().to_lowercase() == "yellow" {
        println!("Player will be yellow. Computer will be red.");
        return ("yellow".to_string(), "red".to_string());
    }
    else {
        println!("Player will be red. Computer will be yellow.");
        return ("red".to_string(), "yellow".to_string());
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

fn update_board() -> () {
    
}

fn check_if_winner() -> bool {
    return false;
}

fn winning_message() {
    println!("Congrats! The winner is ");
}

fn check_playable() -> bool {
    return true;
}
