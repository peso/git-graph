/*! This example shows how to use a different git backend with git-graph,
in this case the gioxide library.
*/

use git_graph::config::create_config;
use git_graph::graph::GitGraph;
use git_graph::print::format::CommitFormat;
use git_graph::print::unicode::print_unicode;
use git_graph::settings::BranchOrder;
use git_graph::settings::BranchSettings;
use git_graph::settings::MergePatterns;
use git_graph::settings::Settings;
use platform_dirs::AppDirs;
use gix_revwalk;

fn main() {
    let settings = get_settings();

    let mut graph = GitGraph::new(repository, &settings)
        .expect("Create empty GitGraph");
    extract(&mut graph, repo_path)
        .expect("Extract commits using git2");

    let (graph_lines, text_lines, _indices) = print_unicode(&graph, &settings).unwrap();
    for (g_line, t_line) in graph_lines.iter().zip(text_lines.iter()) {
        println!(" {}  {}", g_line, t_line);
    }
}

/// Read settings from a config file or create a default config file
fn get_settings() -> Settings {
    let app_dir = AppDirs::new(Some("git-graph"), false).unwrap().config_dir;
    let mut models_dir = app_dir;
    models_dir.push("models");

    create_config(&models_dir).unwrap();

    let model = get_model(
        &repository,
        matches.get_one::<String>("model").map(|s| &s[..]),
        REPO_CONFIG_FILE,
        &models_dir,
    )?;

    Settings {
        reverse_commit_order,
        debug: false,
        colored: false,
        compact: false,
        include_remote: false,
        format: CommitFormat::default(),
        wrapping: None,
        characters: Characters::thin(),
        branch_order: BranchOrder::ShortestFirst(true),
        branches: BranchSettings::from(model).unwrap(),
        merge_patterns: MergePatterns::default(),
    }
}

/// Extract commits using gix and place in GitGraph
fn extract(graph: &mut GitGraph) 
  -> Result<String, String> {
    let repo = RepoProxy::new(repo_path);

    let branch_feed = repo.extract_branches();
    graph.add_branch_heads(branch_feed);

    let commit_feed = repo.extract_commits();
    graph.extend_commits(commit_feed);
}

/// Commits in the repository have 3 levels of awareness
/// (0) unknown. The commit is in the repository but unknown to the proxy
/// (1) identified. Commit hash has been given a synthetic key
/// (2) visited. Parents of the commit are in memory
struct RepoProxy {
    type Oid = graph2::Oid;
    // Map a Graph2::Oid (an index) to a gix Oid (a content hash)
    // Not all commits that have an index have been scanned for parents
    commit_oid: Vec<gix::Oid>,

    /// Map a commit oid index to a list of commit oid index corresponding
    /// to its parents. If commitparents[i].len() == 0, then it has been
    /// identified, but not yet visited 
    commit_parents: Vec<Vec<Oid>>,
}

impl RepoProxy {
    pub fn extract_all() {

    }
}
impl graph2::RepoProxy for RepoProxy {
    /// Test if a commit is in proxy cache. If not, then 
    /// The commit may be loaded at a later time.
    fn in_cache(commit: Oid) -> bool {
        let is_visited = self.commit_parents[commit].len() > 0;
        is_visited
    }

    /// Find the synthetic ids of the parents.
    /// For commit id not loaded, return an empty vec.
    fn parents(child: Oid) -> Vec<Oid> {
        self.commit_parents[child].clone()
    }
}

impl graph2::CommitFeed for RepoProxy {
    
}

impl graph2::BranchFeed for RepoProxy {
    
}

struct CommitInfo {
    oid: git2_OID,
}
    