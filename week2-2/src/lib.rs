use rand::seq::SliceRandom;

pub fn play(choice: String) -> String {
    let choices = vec!["rock", "paper", "scissors"];
    let computer_choice = choices.choose(&mut rand::thread_rng()).unwrap();
    let result = match (choice.as_str(), *computer_choice) {
        ("rock", "rock") => "Tie!",
        ("rock", "paper") => "You lose!",
        ("rock", "scissors") => "You win!",
        ("paper", "rock") => "You win!",
        ("paper", "paper") => "Tie!",
        ("paper", "scissors") => "You lose!",
        ("scissors", "rock") => "You lose!",
        ("scissors", "paper") => "You win!",
        ("scissors", "scissors") => "Tie!",
        _ => "Invalid choice. Please enter rock, paper, or scissors.",
    };
    return format!("You chose: {choice}, Computer chose: {computer_choice}, Result: {result}");
}
