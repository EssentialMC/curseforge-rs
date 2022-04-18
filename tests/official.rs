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
#[ignore]
fn search_iter() {
    use smol::pin;
    use smol::stream::StreamExt;

    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();
        let params = SearchParams::game(GAME_MINECRAFT);

        let projects_iter = client.search_iter(params).await;
        pin!(projects_iter);

        let mut count = 0;

        while let Some(project) = projects_iter.next().await {
            if count >= 150 {
                break;
            }

            match &project {
                Ok(item) => {
                    count += 1;
                    println!("{:#?}", item);
                }
                Err(error) => {
                    eprintln!("Error encountered after {} results!\n{:#?}", count, error)
                }
            }

            assert!(project.is_ok());
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
    use smol::pin;
    use smol::stream::StreamExt;

    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();

        let project_ids = {
            let params = SearchParams::game(GAME_MINECRAFT);
            let projects = client.search_iter(params);
            pin!(projects);

            let mut count = 0_usize;
            let mut ids = Vec::new();

            while let Some(result) = projects.next().await {
                if count >= 3000 {
                    break;
                }

                match &result {
                    Ok(project) => ids.extend([project.id]),
                    Err(error) => eprintln!("{:#?}", error),
                }

                assert!(result.is_ok());
                count += 1;
            }

            ids
        };

        let projects = client.projects(project_ids).await;

        match &projects {
            Ok(projects) => println!("{:#?}", projects),
            Err(error) => eprintln!("{:#?}", error),
        }

        assert!(projects.is_ok())
    });
}
