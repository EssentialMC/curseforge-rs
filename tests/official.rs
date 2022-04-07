use curseforge::official::prelude::*;

const API_BASE: &str = "https://cfproxy.fly.dev/v1/";

#[test]
fn game() {
    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();

        let game = client.game(431).await;

        match &game {
            Ok(game) => println!("{:#?}", game),
            Err(error) => eprintln!("{:#?}", error),
        }

        assert!(game.is_ok());
    });
}

#[test]
fn games() {
    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();

        let games = client.games(&GamesParams::default()).await;

        match &games {
            Ok(games) => println!("{:#?}", games),
            Err(error) => eprintln!("{:#?}", error),
        }

        assert!(games.is_ok());
    });
}
