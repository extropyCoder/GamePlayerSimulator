
struct Player {
    player_number : u8

}

struct Coalition_Set {
    players : Vec<Player>,
    value : u32
}

struct Game_Set {
    coalitions : Vec<Coalition_Set>
}

fn main() {
    println!("Starting Game Player Simulation!");
    // magic happens here
    let player1 = Player{player_number : 1};

    let game_set = create_game_set();

    evaluate_characteristic_function(player1);
    evaluate_shapley(game_set);


    println!("Finished");

}

fn create_game_set() -> Game_Set{
let player = Player{player_number : 0};
let players = vec![player];
let coalition_set = Coalition_Set{players : players,value : 0};
let coalitions = vec![coalition_set];

    return Game_Set{coalitions : coalitions };
}


fn evaluate_characteristic_function(player : Player)->f64{
    return 0.0;
}

fn evaluate_shapley(game_set : Game_Set){

}

fn calcFactorial(n:u64)->u64{
    if n == 0 {
        return 1;
    }
    else {
        return  n*calcFactorial(n-1);
}
}
