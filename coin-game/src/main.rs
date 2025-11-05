use rand::Rng;

fn get_input() -> anyhow::Result<i32> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    input.trim().parse().map_err(anyhow::Error::msg)
}

fn main() {
    let mut coins = 21;
    let mut rng = rand::thread_rng();

    loop {
        let player1 = loop {
            print!("Current coin is {}. Enter number: ", coins);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();

            match get_input() {
                Ok(input) => {
                    if input > 2 || input < 1 {
                        println!("Invalid input!");
                    } else {
                        break input;
                    }
                }
                Err(err) => {
                    println!("{}", err);
                }
            };
        };
        println!("Player 1 take {} coins.", player1);

        coins -= player1;
        if coins <= 0 {
            println!("Player 1 wins!");
            break;
        }

        let player2 = rng.gen_range(1..=2);
        println!("Player 2 take {} coins.", player2);

        coins -= player2;
        if coins <= 0 {
            println!("Player 2 wins!");
            break;
        }
    }
}
