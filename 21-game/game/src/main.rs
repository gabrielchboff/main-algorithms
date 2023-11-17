use rand::prelude::*;
use std::collections::HashMap;
#[derive(Debug)]
struct Menu {
    name: String,
    options: HashMap<u32, String>,
}

impl Menu {
    fn new(name: String, current_options: HashMap<u32, String>) -> Menu {
        Menu {
            name,
            options: current_options,
        }
    }

    fn show_menu(&self) {
        println!("Welcome to {}\n", self.name);
        for (key, value) in &self.options {
            println!("{}. {}\n", key, value);
        }
    }

    fn get_user_input(&self) -> u32 {
        println!("Please select an option: ");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let result: u32 = input.trim().parse::<u32>().unwrap();
        return result;
    }
    fn rules(&self) {
        let rules = r#"
        21 is a two player game, the game is played by choosing a number 
        (1, 2, or 3) to be added to the running total.

        The game is won by the player whose chosen number causes the running total to reach exactly 21.
        The running total starts at zero. One player will be the computer.
        Players alternate supplying a number to be added to the running total.
        "#;
        println!("{}", rules);
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    score: u32,
}

impl Player {
    fn new(name: String) -> Player {
        Player { name, score: 0 }
    }

    fn add_score(&mut self, score: u32) {
        self.score += score
    }

    fn get_score(&self) -> u32 {
        self.score
    }
}

#[derive(Debug)]
struct Game {
    player: Player,
    computer_score: u32,
}

impl Game {
    fn new(current_player: Player) -> Game {
        Game {
            player: current_player,
            computer_score: 0,
        }
    }

    fn play(&mut self) {
        let mut player_turn: String = String::new();
        let mut rng = rand::thread_rng();
        let roll: u32 = rng.gen_range(1..3);

        println!("Welcome {}\n", self.player.name);
        print!("It's your turn. Do you want to roll or pass? (r/p): ");

        std::io::stdin()
            .read_line(&mut player_turn)
            .expect("Failed to read input");

        if player_turn.trim() == "r" {
            println!("You rolled a {}", roll);
            self.player.add_score(roll);
            println!("You now have a score of {}", self.player.get_score());
        } else if player_turn.trim() == "p" {
            println!("You chose to pass. Computer rolled a {}", roll);
            self.computer_score += roll;
            println!("Computer now has a score of {}", self.computer_score);
        }
    }
}

fn main() {
    let mut menu_options: HashMap<u32, String> = HashMap::new();
    menu_options.insert(1, "Play".to_string());
    menu_options.insert(2, "Rules".to_string());
    menu_options.insert(3, "Quit".to_string());

    let menu: Menu = Menu::new("21 Game!".to_string(), menu_options);

    menu.show_menu();
    let input: u32 = menu.get_user_input();
    while input != 3 {
        if input == 1 {
            print!("Please enter your name: ");
            let mut player_name: String = String::new();
            std::io::stdin()
                .read_line(&mut player_name)
                .expect("Failed to read input");
            let player: Player = Player::new(player_name);
            let mut game: Game = Game::new(player);
            game.play();
        } else if input == 2 {
            menu.rules();
            break;
        }
    }
}
