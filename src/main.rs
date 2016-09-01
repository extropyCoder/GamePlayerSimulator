
struct Player {
    player_number : u8,

}

struct GameSet {
    players : Vec<Player>
}

struct CoalitionSet {
    players : Vec<Player>
}

fn main() {
    println!("Starting Game Player Simulation!");
    // magic happens here
    let player1 = Player{player_number : 1};

    evaluate_characteristic_function(player1);
    evaluate_shapley();


    println!("Finished");

}


fn evaluate_characteristic_function(player : Player)->f64{
    return 0.0;
}

fn evaluate_shapley(){





}
