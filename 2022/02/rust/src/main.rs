use std::fs;

static RPS: &[&str] = &["A", "B", "C"];

fn main() {
    let contents: String = fs::read_to_string("../../input.txt").unwrap();

    let mut points = 0;

    for game in contents.split("\n") {
        points += get_points(game)
    }

    println!("{}", points.to_string())
}

fn get_points(game: &str) -> i32 {
    let game_info: Vec<&str> = game.split(" ").collect::<Vec<&str>>();

    println!(
        "enemy - {}, game outcome - {}, my play - {}",
        game_info[0],
        game_info[1],
        get_my_play(game_info[0], game_info[1])
    );

    return get_match_points(game_info[1])
        + get_my_points(&get_my_play(game_info[0], game_info[1]));
}

fn get_my_play(enemy: &str, match_result: &str) -> String {
    let enemy_idx = RPS.iter().position(|e| e.to_owned() == enemy).unwrap();

    match match_result {
        "X" => return RPS[(enemy_idx + 2) % 3].to_owned(),
        "Z" => return RPS[(enemy_idx + 1) % 3].to_owned(),
        _ => return RPS[(enemy_idx)].to_owned(),
    }
}

fn get_my_points(a: &str) -> i32 {
    match a {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!(),
    }
}

fn get_match_points(match_result: &str) -> i32 {
    match match_result {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!(),
    }
}
