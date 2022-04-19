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
    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();
        let projects = sample_search_projects(&client, GAME_MINECRAFT, usize::MAX).await;

        for project in projects {
            println!("{:#?}", project);
        }
    });
}

/// Example performs a request for the data from the first 1000 projects
/// returned from a sample search, by their ID.
#[test]
fn project() {
    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();
        let projects = sample_search_projects(&client, GAME_MINECRAFT, 1000).await;
        let project_ids = projects.into_iter().map(|project| project.id);

        for project in project_ids {
            let result = client.project(project).await;

            match result {
                Ok(project) => println!("{:#?}", project),
                Err(error) => panic!("{:#?}", error),
            }
        }
    });
}

/// Example performs a search for the first 3000 projects for the game
/// Minecraft, and then makes a single batched request to get data for each of
/// those results by their project ID.
#[test]
fn projects() {
    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();

        let projects = sample_search_projects(&client, GAME_MINECRAFT, 3000).await;
        let project_ids = projects.into_iter().map(|project| project.id);
        let result = client.projects(project_ids).await;

        match result {
            Ok(projects) => println!("{:#?}", projects),
            Err(error) => panic!("{:#?}", error),
        }
    });
}

/// Example makes a request with default parameters for getting featured
/// projects for te game Minecraft.
#[test]
fn featured_projects() {
    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();

        let body = FeaturedProjectsBody::game(GAME_MINECRAFT);
        let result = client.featured_projects(&body).await;

        match result {
            Ok(featured) => println!("{:#?}", featured),
            Err(error) => panic!("{:#?}", error),
        }
    });
}

/// Example performs a request to get a file by ID for each file of the first
/// 150 projects returned from a search.
#[test]
fn project_file() {
    use std::collections::HashMap;

    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();

        let projects = sample_search_projects(&client, GAME_MINECRAFT, 150).await;
        let project_files = projects
            .into_iter()
            .map(|project| {
                (
                    project.id,
                    project.latest_files.into_iter().map(|file| file.id),
                )
            })
            .collect::<HashMap<_, _>>();

        for (project, files) in project_files.into_iter() {
            for file in files {
                let result = client.project_file(project, file).await;

                match result {
                    Ok(file) => println!("{:#?}", file),
                    Err(error) => panic!("{:#?}", error),
                }
            }
        }
    });
}

/// Example makes requests for the first 3000 projects from a sample search and
/// retrieves the files for each based on empty or default parameters.
#[test]
fn project_files() {
    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();

        let projects = sample_search_projects(&client, GAME_MINECRAFT, 3000).await;
        let project_ids = projects.into_iter().map(|project| project.id);

        for project in project_ids {
            let result = client.project_files(project, None).await;

            match result {
                Ok(projects) => println!("{:#?}", projects),
                Err(error) => panic!("{:#?}", error),
            }
        }
    });
}

/// Example makes requests for the first 3000 projects from a sample search and
/// retrieves the files for each based on empty or default parameters.
#[test]
fn project_files_iter() {
    use smol::pin;
    use smol::stream::StreamExt;

    smol::block_on(async {
        let client = Client::new(API_BASE, None).unwrap();

        let projects = sample_search_projects(&client, GAME_MINECRAFT, 3000).await;
        let project_ids = projects.into_iter().map(|project| project.id);

        for project in project_ids {
            let files = client.project_files_iter(project, None);
            pin!(files);

            while let Some(result) = files.next().await {
                match result {
                    Ok(file) => println!("{:#?}", file),
                    Err(error) => panic!("{:#?}", error),
                }
            }
        }
    });
}

/// Utility function to reduce duplication. Many tests require data from
/// projects so this performs the necessary search to acquire sample data.
async fn sample_search_projects(client: &Client, game_id: i32, amount: usize) -> Vec<Project> {
    use smol::pin;
    use smol::stream::StreamExt;

    let params = SearchParams::game(game_id);
    let search = client.search_iter(params);
    pin!(search);

    let mut count = 0_usize;
    let mut projects = Vec::new();

    while let Some(result) = search.next().await {
        if count >= amount {
            break;
        }

        match result {
            Ok(project) => projects.extend([project]),
            Err(error) => panic!("{:#?}", error),
        }

        count += 1;
    }

    projects
}
