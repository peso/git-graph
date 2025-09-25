/*! A graph structure representing the history of a VCS repository.

This module is generic to the version control system. This means
it can be used for several kinds of VCS as well as for different
backends for VCS. The primary motivation was to allow both gix and git2
backends for git repositories.

git-graph visualizes branches as a vertical line. Only
the primary parent of a commit can be on the same branch as the
commit. Horizontal lines represent forks (multiple children) or
merges (multiple parents), and show the remaining parent relations.

To trace the branches in a repository use [BranchGraph]. You will have
to provide imlementations of traits [CommitFeed] and [BranchFeed].
*/

use std::collections::BinaryHeap;
use std::collections::HashMap;


//
//  Data structure
//


// The identifier of a commit. Managed externally
type Oid = usize;

// The identifier of a branch. Managed by BranchGraph
type Bid = usize;

// The level of persistence when a branch traces its ancestors
type Pers = usize;

/// A collection of branch traces. Built for incremental updates
pub struct BranchGraph {
    /// Which branch has claimed this commit
    commit_branch: HashMap<Oid,Bid>,
    /// All defined branches
    branch: Vec<BranchTrace>,
    /// Maps a commit id to the branch reaching for it
    open_branch: HashMap<Oid,Bid>,
}


/// Represents a branch
pub struct BranchTrace {
    // The yougest descendant in the branch
    pub target: Oid,
    // The oldest ancestor in the branch
    pub source: Oid,
    // If a branch is open, then the ancestor has not yet been examined.
    // This means that there may be an even older ancestor in the trace
    // An open branch treats the primary parent of source as potentially
    // in the branch as well. If next_source is none, then the branch
    // is closed.
    pub next_source: Option<Oid>,

    // Name of branch. Used to determine persistence
    pub name: String,
    // Persistence of branch means how hard it will work for claiming
    // ancestors. The highest persistence wins the fork race.
    pub persistence: Pers,
}

/// A builder that updates a BranchGraph with more information.
/// This is used to provide fine grained control over how much effort
/// is spent on traversing the graph.
pub struct BranchGraphBuilder<'a> {
    /// The branch graph that is being updated
    graph: &'a mut BranchGraph,

    /// A queue of commits that will be examined
    queue: BinaryHeap<Oid>,
}


/* TODO maybe add a short versin?
type BranchVis = BranchVisualization;
*/

/// Branch properties used for visualization.
pub struct BranchVisualization {
    /// The branch's column group (left to right)
    pub order_group: usize,
    /// The branch's merge target column group (left to right)
    pub target_order_group: Option<usize>,
    /// The branch's source branch column group (left to right)
    pub source_order_group: Option<usize>,
    /// The branch's terminal color (index in 256-color palette)
    pub term_color: u8,
    /// SVG color (name or RGB in hex annotation)
    pub svg_color: String,
    /// The column the branch is located in
    pub column: Option<usize>,
}


//
//  Interface
//

/// Information about a single commit
pub trait CommitInfo {
    /// The commit id synthetic key.
    /// This should map to a real commmit id and be unique
    /// across all CommitFeed sent to the same BranchGraph.
    fn id() -> Oid;

    /// The commit id key of all parents.
    fn parents() -> Vec<Oid>;
}

/// Information about a single branch
pub trait BranchInfo {
    fn name(&self) -> String;
    fn commit_id(&self) -> Oid;
    fn persistence(&self) -> Pers;
}

/// A subset of all commits in the repository.
pub trait RepoProxy {
    /// Test if a commit is in proxy cache. If not, then 
    /// The commit may be loaded at a later time.
    fn in_cache(commit: Oid) -> bool;
    /// Find the synthetic ids of the parents.
    /// For commit id not loaded, return an empty vec.
    fn parents(child: Oid) -> Vec<Oid>;
}

/// Provide commit information to a branch graph
pub trait CommitFeed: RepoProxy + Iterator<Item: CommitInfo> {
}

/// Provide branch information to a branch graph
pub trait BranchFeed: RepoProxy + Iterator<Item: BranchInfo> {
}


//
//  Implementation
//

impl BranchGraph {
    /// Create an empty branch graph
    pub fn empty() -> Self {
        BranchGraph {
            commit_branch: HashMap::new(),
            branch: vec![],
            open_branch: HashMap::<_,_>::new(),
        }
    }

    /// Add more labels to graph. This will cause a recomputation that
    /// worst case affect the entire graph.
    pub fn add_branch_heads<F: BranchFeed>(&mut self, branch_feed: F) {
    }

    /// Recompute structure given more Extend a branch graph with more commits
    pub fn extend_commits<CF: CommitFeed>(&mut self, commit_feed: CF) {
    }

    /// Merge two branch graphs. Assume that self is larger,
    //  was made first, with younger commits.
    //  other is smaller and contain ancestors relative to self.
    pub fn consume(&mut self, other: Self) {
    }
}

impl<'a> BranchGraphBuilder<'a> {
    /// Create an empty branch graph builder
    pub fn new(graph: &'a mut BranchGraph) -> Self {
        Self {
            graph,
            queue: BinaryHeap::new(),
        }
    }

    /// Add a commit to the 
    pub fn push(commit_id: Oid) {

    }

    /// Process one iteration of branch extension.
    /// Return true if there is still work to be done, and you must call
    /// it again.
    pub fn iterate(&mut self) -> bool {
        // TODO Process one iteration

        // Return true if more work should be done
        self.queue.peek().is_some()
    }

}
