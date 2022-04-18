use curseforge::official::prelude::*;

const API_BASE: &str = "https://cfproxy.fly.dev/v1/";
const GAME_TERRARIA: i32 = 431;
const GAME_MINECRAFT: i32 = 432;

/// Example performs a request for the data for a specific game by ID.
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

/// Example performs a request for all games supported by the API.
#[test]
fn games() {
    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();
        let params = GamesParams::default();
        let games = client.games(&params).await;

        match &games {
            Ok(games) => println!("{:#?}", games),
            Err(error) => eprintln!("{:#?}", error),
        }

        assert!(games.is_ok());
    });
}

/// Example performs a request for the versions of a game by its ID. The `type`
/// field corresponds to a "version type", for example, the version of a
/// modloader, or a major release for a game.
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

/// Example performs a request for getting a paginated response for the "version
/// types" (similar to categories, but for version numbers).
#[test]
fn game_version_types() {
    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();
        let params = GamesParams::default();
        let games = client.games(&params).await;

        match &games {
            Ok(games) => println!("{:#?}", games),
            Err(error) => eprintln!("{:#?}", error),
        }

        assert!(games.is_ok());
    });
}

/// Example performs a request for getting a list of project categories for the
/// game Minecraft.
#[test]
fn categories() {
    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();
        let params = CategoriesParams::game(GAME_MINECRAFT);
        let categories = client.categories(&params).await;

        match &categories {
            Ok(categories) => println!("{:#?}", categories),
            Err(error) => eprintln!("{:#?}", error),
        }

        assert!(categories.is_ok());
    });
}

/// Example demonstrates searching for projects for the game Minecraft, and does
/// not paginate over the data returned. For pagination, see `search_iter`. This
/// demonstrates proper deserialization into the `Pagination` type.
#[test]
fn search() {
    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();
        let params = SearchParams::game(GAME_MINECRAFT);
        let result = client.search(&params).await;

        match &result {
            Ok(response) => println!("{:#?}", response),
            Err(error) => eprintln!("{:#?}", error),
        }

        assert!(result.is_ok());
    });
}

/// Example asynchronously paginates over the maximum allowed search results
/// (10,000) for the game Minecraft. This demonstrates proper deserialization
/// into the wrapper's types as well as the proper usage of `PaginatedStream`.
#[test]
fn search_iter() {
    use smol::pin;
    use smol::stream::StreamExt;

    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();
        let params = SearchParams::game(GAME_MINECRAFT);
        let projects = client.search_iter(params);
        pin!(projects);

        let mut count = 0_usize;

        while let Some(result) = projects.next().await {
            match &result {
                Ok(project) => {
                    println!("{:#?}", project);
                }
                Err(error) => {
                    eprintln!(
                        "Stream closed unexpectedly after {} results!\n{:#?}",
                        count, error
                    )
                }
            }

            assert!(result.is_ok());
            count += 1;
        }
    });
}

/// Example performs a request for the data from one project ID, Mouse Tweaks.
/// To demonstrate that the wrapper's deserializing types are correct see the
/// more robust example, `projects`.
#[test]
fn project() {
    const MOUSE_TWEAKS_MOD_ID: i32 = 60089;

    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();
        let project = client.project(MOUSE_TWEAKS_MOD_ID).await;

        match &project {
            Ok(project) => println!("{:#?}", project),
            Err(error) => eprintln!("{:#?}", error),
        }

        assert!(project.is_ok());
    });
}

/// Example performs a search for the first 3000 projects for the game
/// Minecraft, and then makes a single batched request to get data for each of
/// those results by their project ID.
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
