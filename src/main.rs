use std::io;

fn main() {
    welcome_message();

    const N: usize = 6;
    const M: usize = 7;

    let mut _board = [[0 as char; N] ; M];

    assign_players();

    loop {
        let winner_exists = check_if_winner();

        _board = update_board(0, 0, 'X', _board);

        // print board
        print_board(_board);

        if winner_exists {
            winning_message();
            break;
        }
    }
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

fn print_board( _board: [[char;6];7] ) -> () {
    for _x in 0..6 {
        for _y in 0..6 {
            print!("| {} ", _board[_x][_y]);
        }
        println!("|");
    }
    println!();
}

fn update_board(x: usize, y: usize, c: char, mut _board: [[char;6];7]) -> [[char;6];7] {
    _board[x][y] = c;
    return _board;
}

fn check_if_winner() -> bool {
    return true;
}

fn winning_message() {
    println!("Congrats! The winner is ");
}

fn check_playable() -> bool {
    return true;
}
