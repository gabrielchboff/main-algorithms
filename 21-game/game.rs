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
    fn new(current_player: String) -> Game {
        Game {
            player: Player::new(current_player),
            computer_score: 0,
        }
    }

    fn play(&self) {
        println!("Welcome {}\n", self.player.name);

    }

fn main() {
    let mut menu_options: HashMap<u32, String> = HashMap::new();
    menu_options.insert(1, "Play".to_string());
    menu_options.insert(2, "Rules".to_string());
    menu_options.insert(3, "Quit".to_string());

    let menu = Menu::new("21 Game!".to_string(), menu_options);


    menu.show_menu();
    let mut input: u32 = menu.get_user_input();
    while input != 3 {
        if input == 1 {
        } else {
        }
    }
}
