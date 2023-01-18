use std::io;
use rand::Rng;
enum RPS<> {
    Rock,
    Paper,
    Scissor,
}
impl PartialEq for RPS{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RPS::Rock, RPS::Rock) => true,
            (RPS::Paper, RPS::Paper) => true,
            (RPS::Scissor, RPS::Scissor) => true,
            _ => false,
        }
    }
}

fn npc_choose() -> RPS {
    let option_one = RPS::Rock;
    let option_two = RPS::Paper;
    let option_three = RPS::Scissor;
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(1..3);
    return match n {
      1 => option_one,
      2 => option_two,
      3 => option_three,
      _ => panic!(),
    }
}

fn player_choose(n: i32) -> RPS {
    let option_one = RPS::Rock;
    let option_two = RPS::Paper;
    let option_three = RPS::Scissor;
    return match n {
        1 => option_one,
        2 => option_two,
        3 => option_three,
        _ => panic!(),
    }
}

fn who_wins(n: i32) -> &'static str {
    let option_one = RPS::Rock;
    let option_two = RPS::Paper;
    let option_three = RPS::Scissor;
    let npc = npc_choose();
    let player = player_choose(n);
    return if player == option_one && npc == option_three {
        "Player WINS"
    } else if player == option_one && npc == option_two {
        "Player LOSS"
    } else {
        "DRAW"
    }


}


fn main() {
    println!("please choose a hand\n 1 for rock\n 2 for paper\n 3 for scissor" );
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("ERROR");
    let number: i32 = number.trim().parse().expect("ERROR");
    println!("{}",who_wins(number));
}




