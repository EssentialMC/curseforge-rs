use curseforge::official::prelude::*;

const API_BASE: &str = "https://cfproxy.fly.dev/v1/";

#[test]
fn games() {
    smol::block_on(async {
        let games: Result<GamesResponse, _>;

        let mut config = surf::Config::new();
        config = config.set_base_url(surf::Url::parse(API_BASE).unwrap());

        // config = config.add_header("Accept", "application/json").unwrap();

        let client = surf::Client::try_from(config).unwrap();
        let response = client.get("games").recv_string().await.unwrap();

        let mut deser = serde_json::Deserializer::from_str(&response);

        games = serde_path_to_error::deserialize(&mut deser);

        match &games {
            Ok(games) => println!("{:#?}", games),
            Err(error) => eprintln!("{:#?}", error),
        }

        assert!(games.is_ok());
    });
}
