use curseforge::official::prelude::*;
use once_cell::sync::Lazy;

/// The API base that will be used if a token is not provided.
/// This is an unofficial proxy that works without a token.
static PROXY_API_BASE: &str = "https://api.curse.tools/v1/cf/";

/// The name of the environment variable to get the CurseForge token from.
static TOKEN_VARIABLE: &str = "CURSEFORGE_API_TOKEN";

/// Settings are lowered to reduce API spam.
static CLIENT_OPTIONS: ClientOptions = ClientOptions {
    // This is the maximum number of client connections allowed for the host.
    // Increasing this number may result in denial errors.
    max_connections: 1,
};

const GAME_TERRARIA: i32 = 431;
const GAME_MINECRAFT: i32 = 432;

static CLIENT: Lazy<Client> = Lazy::new(|| match std::env::var(TOKEN_VARIABLE) {
    Ok(token) => {
        eprintln!("Using official CurseForge API with token.");
        Client::new(e::DEFAULT_API_BASE, Some(token), Some(&CLIENT_OPTIONS)).unwrap()
    }
    Err(_) => {
        eprintln!("Using proxy for CurseForge API without token.");
        Client::new(PROXY_API_BASE, None, Some(&CLIENT_OPTIONS)).unwrap()
    }
});

static SAMPLE_PROJECTS: Lazy<Vec<Project>> = Lazy::new(|| {
    smol::block_on(async {
        use smol::pin;
        use smol::stream::StreamExt;

        let params = ProjectSearchParams::game(GAME_MINECRAFT);
        let search = CLIENT.search_projects_iter(params);
        pin!(search);

        let mut projects = Vec::new();

        while let Some(result) = search.next().await {
            match result {
                Ok(project) => projects.extend([project]),
                Err(error) => panic!("{}", error),
            }
        }

        projects
    })
});

/// Example performs a request for the data for a specific game by ID.
#[test]
fn game() {
    smol::block_on(async {
        let game = CLIENT.game(GAME_TERRARIA).await;

        match &game {
            Ok(_game) => (), /* println!("{:#?}", game) */
            Err(error) => panic!("{}", error),
        }
    });
}

/// Example performs a request for all games supported by the API.
#[test]
fn games() {
    smol::block_on(async {
        let params = GamesParams::default();
        let games = CLIENT.games(&params).await;

        match &games {
            Ok(_games) => (), /* println!("{:#?}", games) */
            Err(error) => panic!("{}", error),
        }
    });
}

/// Example performs a request for the versions of a game by its ID. The `type`
/// field corresponds to a "version type", for example, the version of a
/// modloader, or a major release for a game.
#[test]
fn game_versions() {
    smol::block_on(async {
        let versions = CLIENT.game_versions(GAME_MINECRAFT).await;

        match &versions {
            Ok(_games) => (), /* println!("{:#?}", games) */
            Err(error) => panic!("{}", error),
        }
    });
}

/// Example performs a request for getting a paginated response for the "version
/// types" (similar to categories, but for version numbers).
#[test]
fn game_version_types() {
    smol::block_on(async {
        let params = GamesParams::default();
        let games = CLIENT.games(&params).await;

        match &games {
            Ok(_games) => (), /* println!("{:#?}", games) */
            Err(error) => panic!("{}", error),
        }
    });
}

/// Example performs a request for getting a list of project categories for the
/// game Minecraft.
#[test]
fn categories() {
    smol::block_on(async {
        let params = CategoriesParams::game(GAME_MINECRAFT);
        let categories = CLIENT.categories(&params).await;

        match &categories {
            Ok(_categories) => (), /* println!("{:#?}", categories) */
            Err(error) => panic!("{}", error),
        }
    });
}

/// Example demonstrates searching for projects for the game Minecraft, and does
/// not paginate over the data returned. For pagination, see `search_iter`. This
/// demonstrates proper deserialization into the `Pagination` type.
#[test]
fn search_projects() {
    smol::block_on(async {
        let params = ProjectSearchParams::game(GAME_MINECRAFT);
        let result = CLIENT.search_projects(&params).await;

        match &result {
            Ok(_response) => (), /* println!("{:#?}", response) */
            Err(error) => panic!("{}", error),
        }
    });
}

/// Example asynchronously paginates over the maximum allowed search results
/// (10,000) for the game Minecraft. This demonstrates proper deserialization
/// into the wrapper's types as well as the proper usage of `PaginatedStream`.
#[test]
fn search_projects_iter() {
    assert!(!SAMPLE_PROJECTS.is_empty())
}

/// Example performs a request for the data from the first 500 projects
/// returned from a sample search, by their ID.
#[test]
fn project() {
    smol::block_on(async {
        let projects = &SAMPLE_PROJECTS[..500];
        let project_ids = projects.iter().map(|project| project.id);

        for project in project_ids {
            let result = CLIENT.project(project).await;

            match result {
                Ok(_project) => (), /* println!("{:#?}", project) */
                Err(error) => panic!("{}", error),
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
        let projects = &SAMPLE_PROJECTS[..3000];
        let project_ids = projects.iter().map(|project| project.id);
        let result = CLIENT.projects(project_ids).await;

        match result {
            Ok(_projects) => (), /* println!("{:#?}", projects) */
            Err(error) => panic!("{}", error),
        }
    });
}

/// Example makes a request with default parameters for getting featured
/// projects for te game Minecraft.
#[test]
fn featured_projects() {
    smol::block_on(async {
        let body = FeaturedProjectsBody::game(GAME_MINECRAFT);
        let result = CLIENT.featured_projects(&body).await;

        match result {
            Ok(_featured) => (), /* println!("{:#?}", featured) */
            Err(error) => panic!("{}", error),
        }
    });
}

/// Example makes a request to get the project descriptions for the first 150
/// results from a sample search.
#[test]
fn project_description() {
    smol::block_on(async {
        let projects = &SAMPLE_PROJECTS[..150];
        let project_ids = projects.iter().map(|project| project.id);

        for project in project_ids {
            let result = CLIENT.project_description(project).await;
            // let result = result.map(|description| description.data);
            match result {
                Ok(_description) => (), /* println!("{}", **description) */
                Err(error) => panic!("{}", error),
            }
        }
    });
}

/// Example performs a request to get a file by ID for each file of the first
/// 150 projects returned from a search.
#[test]
fn project_file() {
    use std::collections::HashMap;

    smol::block_on(async {
        let projects = &SAMPLE_PROJECTS[..150];
        let project_files = projects
            .iter()
            .map(|project| (project.id, project.latest_files.iter().map(|file| file.id)))
            .collect::<HashMap<_, _>>();

        for (project, files) in project_files.into_iter() {
            for file in files {
                let result = CLIENT.project_file(project, file).await;

                match result {
                    Ok(_file) => (), /* println!("{:#?}", file) */
                    Err(error) => panic!("{}", error),
                }
            }
        }
    });
}

#[test]
fn project_file_by_id() {
    smol::block_on(async {
        let projects = &SAMPLE_PROJECTS[..150];
        let files = projects
            .iter()
            .flat_map(|project| project.latest_files.iter().map(|file| file.id));

        for file in files {
            let result = CLIENT.project_file_by_id(file).await;

            match result {
                Ok(_file) => (), /* println!("{:#?}", file) */
                Err(error) => panic!("{}", error),
            }
        }
    });
}

/// Example makes requests for the first 3000 projects from a sample search and
/// retrieves the files for each based on empty or default parameters.
#[test]
fn project_files() {
    smol::block_on(async {
        let params = ProjectFilesParams::default();

        let projects = &SAMPLE_PROJECTS[..3000];
        let project_ids = projects.iter().map(|project| project.id);

        for project in project_ids {
            let result = CLIENT.project_files(project, &params).await;

            match result {
                Ok(_projects) => (), /* println!("{:#?}", projects) */
                Err(error) => panic!("{}", error),
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
        let params = ProjectFilesParams::default();

        let projects = &SAMPLE_PROJECTS[..3000];
        let project_ids = projects.iter().map(|project| project.id);

        for project in project_ids {
            let files = CLIENT.project_files_iter(project, params.clone());
            pin!(files);

            while let Some(result) = files.next().await {
                match result {
                    Ok(_file) => (), /* println!("{:#?}", file) */
                    Err(error) => panic!("{}", error),
                }
            }
        }
    });
}

/// Example makes requests for every project's main file for the first 3000
/// projects returned form a sample search.
#[test]
fn project_files_by_ids() {
    smol::block_on(async {
        let projects = &SAMPLE_PROJECTS[..3000];
        let file_ids = projects.iter().map(|project| project.main_file_id);

        let result = CLIENT.project_files_by_ids(file_ids).await;

        match result {
            Ok(_files) => {
                // for _file in files {
                //     (); /* println!("{:#?}", file) */
                // }
            }
            Err(error) => panic!("{}", error),
        }
    });
}

/// Example performs a request to get file changelogs for the main file for each
/// project returned from a sample search of the first 500 projects.
#[test]
fn project_file_changelog() {
    use std::collections::HashMap;

    smol::block_on(async {
        let projects = &SAMPLE_PROJECTS[..500];
        let project_files = projects
            .iter()
            .map(|project| (project.id, project.main_file_id))
            .collect::<HashMap<_, _>>();

        for (project, file) in project_files.into_iter() {
            let result = CLIENT.project_file_changelog(project, file).await;

            match result {
                Ok(_changelog) => (), /* println!("{}", changelog) */
                Err(error) => panic!("{}", error),
            }
        }
    });
}

/// Example performs a request to get file changelogs for the main file for each
/// project returned from a sample search of the first 500 projects.
#[test]
fn project_file_download_url() {
    smol::block_on(async {
        let projects = &SAMPLE_PROJECTS[..500];
        let projects_files = projects.iter().filter_map(|project| {
            if let Some(false) = project.allow_mod_distribution {
                None
            } else {
                Some((project.id, project.main_file_id))
            }
        });

        for (project, file) in projects_files {
            let result = CLIENT.project_file_download_url(project, file).await;

            match result {
                Ok(_download) => (), /* println!("{}", download) */
                Err(error) => panic!("{}", error),
            }
        }
    });
}
