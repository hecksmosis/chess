use std::io::Write;

use chess::{Coords, Game};

fn main() {
    let mut game = Game::default();
    println!("{}", game);

    loop {
        let mut valid_input = false;
        let mut coords = Coords::default();

        while !valid_input {
            print!("Enter move: ");
            std::io::stdout().flush().unwrap();
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            match Coords::build(input.trim()) {
                Ok(parsed_coords) => {
                    coords = parsed_coords;
                    println!("x:{}, y:{}, piece:{:?}", coords.x, coords.y, coords.piece);
                    valid_input = true;
                }
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            }
        }

        match game.make_move(coords) {
            Ok(_) => {
                println!("turn: {}, ", game.turn);
                println!("{}", game);
                println!(
                    "White in check?: {}, black in check?: {}",
                    game.is_king_in_check()[0],
                    game.is_king_in_check()[1]
                );
                println!("{}", game);
                let last_bit_mask = 0b0000_0001;
                game.turn ^= last_bit_mask;
                println!("turn at end: {}", game.turn);
            }
            Err(e) => {
                println!("{}, try again", e);
                continue;
            }
        };
    }
}
