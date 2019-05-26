use std::io;

fn main() {
    welcome_message();

    const N: usize = 6;
    const M: usize = 7;

    let mut _board = [[0 as char; N] ; M];

    assign_players();

    loop {
        /*
        For now, X will be "red"
        O will be "yellow"
        */
        loop {
            let mut row_input = String::new();

            // ask for input -- x
            println!("Choose an row (1-6): ");
            io::stdin().read_line(&mut row_input)
                .ok()
                .expect("Failed to read line");
            let row_input: usize = row_input.trim().parse()
                .expect("Please type a number!");

            let row = row_input - 1;

            // check if position is valid
            if is_playable(row, _board) {
                _board = update_board(row, 'X', _board);
                break;
            }
        }

        // print board
        print_board(_board);

        // check if winner exists
        if winner_exists() {
            winning_message("No one :(");
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

fn last_empty_space(row: usize, mut _board: [[char;6];7]) -> usize {
    let mut current_space = 0;
    loop {
        if _board[current_space][row].to_string() != "X" && _board[current_space][row].to_string() != "O" {
            break;
        } else if current_space == 0 {

        } else {
            current_space -= 1;
        }
    }
    return current_space;
}

fn is_playable(row: usize, mut _board: [[char;6];7]) -> bool {
    let x = last_empty_space(row, _board);
    let y = row;
    return _board[x][y].to_string() != "X" && _board[x][y].to_string() != "O";
}

fn update_board(row: usize, c: char, mut _board: [[char;6];7]) -> [[char;6];7] {
    let x = last_empty_space(row, _board);
    let y = row;
    _board[x][y] = c;
    return _board;
}

fn winner_exists() -> bool {
    return false;
}

fn winning_message(winner_name: &str) {
    println!("Congrats! The winner is {}", winner_name);
}

