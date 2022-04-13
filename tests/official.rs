use curseforge::official::prelude::*;

const API_BASE: &str = "https://cfproxy.fly.dev/v1/";
const GAME_TERRARIA: i32 = 431;
const GAME_MINECRAFT: i32 = 432;

#[test]
fn game() {
    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();

        let game = client.game(GAME_TERRARIA).await;

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

#[test]
fn game_versions() {
    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();

        let versions = client.game_versions(GAME_MINECRAFT).await;

        match &versions {
            Ok(games) => println!("{:#?}", games),
            Err(error) => eprintln!("{:#?}", error),
        }

        assert!(versions.is_ok());
    });
}

#[test]
fn game_version_types() {
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

#[test]
fn categories() {
    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();

        let categories = client
            .categories(&CategoriesParams::game(GAME_MINECRAFT))
            .await;

        match &categories {
            Ok(categories) => println!("{:#?}", categories),
            Err(error) => eprintln!("{:#?}", error),
        }

        assert!(categories.is_ok());
    });
}

#[test]
fn search() {
    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();

        let params = SearchParams::game(GAME_MINECRAFT);

        // params.page_size = Some(1);

        let mods = client.search(&params).await;

        match &mods {
            Ok(categories) => println!("{:#?}", categories),
            Err(error) => eprintln!("{:#?}", error),
        }

        assert!(mods.is_ok());
    });
}

#[test]
fn search_iter() {
    use std::io::Write;

    use smol::pin;
    use smol::stream::StreamExt;

    let client = Client::new(API_BASE, None).unwrap();
    let params = SearchParams::game(GAME_MINECRAFT);

    // params.index = Some(5000);

    smol::block_on(async {
        let mods_iter = client.search_iter(params).await;
        pin!(mods_iter);

        let mut count = 0;

        while let Some(item) = mods_iter.next().await {
            match &item {
                Ok(item) => {
                    count += 1;
                    let mut file = std::fs::File::create(&format!(
                        "./target/tests/search_mods_iter/{}.json",
                        item.slug
                    ))
                    .unwrap();
                    file.write_all(serde_json::to_vec(&item).unwrap().as_slice())
                        .unwrap();
                }
                Err(error) => {
                    eprintln!("Error encountered after {} results!\n{:#?}", count, error)
                }
            }

            assert!(item.is_ok());
        }
    });
}

#[test]
fn project() {
    const MOUSE_TWEAKS_MOD_ID: i32 = 60089;

    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();

        let addon = client.project(MOUSE_TWEAKS_MOD_ID).await;

        match &addon {
            Ok(categories) => println!("{:#?}", categories),
            Err(error) => eprintln!("{:#?}", error),
        }

        assert!(addon.is_ok());
    });
}

#[test]
fn projects() {
    let client = Client::new(API_BASE, None).unwrap();
    let mod_ids = [269024, 60089, 398022, 381583];

    smol::block_on(async {
        let projects = client.projects(mod_ids).await;

        match &projects {
            Ok(projects) => println!("{:#?}", projects),
            Err(error) => eprintln!("{:#?}", error),
        }

        assert!(projects.is_ok())
    });
}
