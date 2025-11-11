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

/// A builder that updates a BranchGraph with information from a RepoProxy.
/// This is used to provide fine grained control over how much effort
/// is spent on traversing the graph.
///
/// Note: The present algorithm only allows insertion, not removal. This
/// means that a correctness is not guaranteed if the user removes
/// a branch or rebase commits or change persistence configuration.
/// The workaround is to do a full rebuild.
pub struct BranchGraphBuilder<'a> {
    /// The repository to extract information from
    repo: &'a dyn RepoProxy,

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

/// A subset of all commits in the repository.
pub trait RepoProxy {
    /// Test if a commit is in proxy cache. If not, then 
    /// The commit may be loaded at a later time.
    fn in_cache(&self, commit: Oid) -> bool;
    /// Find the synthetic ids of the parents.
    /// For commit id not loaded, return an empty vec.
    fn parents(&self, child: Oid) -> Vec<Oid>;

    /// Find the synthetic ids of the parents, and their branch name.
    /// The branch name is not used for the first parent.
    /// Any branch name set to the empty string will be ignored.
    fn parents_and_names(&self, child: Oid) -> Vec<(Oid, String)>;

    /// Determine the persistence of a banch name
    fn name_persistence(&self, name: &str) -> Pers;
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
        }
    }
}

impl<'a> BranchGraphBuilder<'a> {
    /// Create an empty branch graph builder
    pub fn new(repo: &'a dyn RepoProxy, graph: &'a mut BranchGraph) -> Self {
        Self {
            repo,
            graph,
            queue: BinaryHeap::new(),
        }
    }

    /// Add a commit to the queue
    pub fn push(&mut self, commit_id: Oid) {
        self.queue.push(commit_id);
    }

    /// Process one iteration of branch extension.
    /// Return true if there is still work to be done, and you must call
    /// it again.
    pub fn iterate(&mut self) -> bool {
        // Get commit to expand
        let Some(cur_commit_id) = self.queue.pop() 
        else {
            return false;
        };

        let parents = self.repo.parents_and_names(cur_commit_id);

        // Current branch grows into first parent,
        // the remaining parents start new branches.
        self.expand_branch(cur_commit_id, parents.get(0).cloned());
        self.process_merge(cur_commit_id, &parents[1..]);

        // Return true if more work should be done
        self.queue.peek().is_some()
    }

    //
    //  Internal functions
    //

    // Expand the branch at child into parent
    fn expand_branch(&mut self, child: Oid, parent: Option<(Oid, String)>) {
        // If child has no parents at all, we are done.
        let Some((parent, _name)) = parent else { return; };

        // Determine branch of commit
        let mut cur_branch = self.graph.commit_branch.get(&child);
        // 1. If current commit has a label,
        // terminate the branch and start a new branch
        // 2. If no branch at this commit, start a
        //  new anonymous branch
        // 3. Otherwise, 

        // TODO - write code

    }

    /// Create a branch for each non-primary parent
    fn process_merge(&mut self, child: Oid, parent_list: &[(Oid, String)]) {
        /*
        // Rust borrow system does really not play nice with closures
        // I need to take the graph and give it back, to avoid a borrow
        // on self.
        let graph = std::mem::take(self.graph);

        // Find child branch
        let child_branch_id = graph.commit_branch.get(&child);
        let child_branch = child_branch_id
            .and_then(|bid| graph.branch.get(*bid));

        self.graph = graph;
        */

        // Or avoid clojures, which means less trouble

        // Find child branch
        let child_branch_id = self.graph.commit_branch.get(&child);
        let child_branch = if let Some(bid) = child_branch_id {
            self.graph.branch.get(*bid)
        } else { None };


        // Update branches for each non-primary parent
        for (parent, extracted_name) in parent_list {

            // Valid parent branch?
            /*
            let branch_opt: Option<&BranchTrace> = 
                graph.commit_branch.get(parent)
                .and_then(|bid| graph.branch.get(*bid));
            */
            /* Attempt #2
            let parent_branch_id = self.graph.commit_branch.get(parent);
            let branch_opt = if let Some(bid) = parent_branch_id {
                self.graph.branch.get(*bid).clone()
            } else { None };
            */
            /* Attempt #3, Gemini suggestion */
            let branch_opt = {
                let parent_branch_id = 
                    self.graph.commit_branch.get(parent).clone();
                if let Some(bid) = parent_branch_id {
                    self.graph.branch.get(*bid).clone()
                } else { 
                    None
                }
            };

                
            // Valid extracted parent name?
            let name_opt = if extracted_name == "" { None }
            else { Some(extracted_name) };

            //
            // Process parent

            let Some(parent_branch) = branch_opt else {
                // No existing branch at parent so create one
                let ANONYMOUS: &String = &String::from("anonymous");
                let new_branch_name = name_opt.unwrap_or(ANONYMOUS);
                self.new_branch(*parent, new_branch_name);
                continue;
            };

            let Some(parent_name) = name_opt else {
                // Parent already has a branch and child gave no new name
                // so keep everything as it is
                continue;
            };

            // Parent already has a branch and child did provide a name

            // Determine if new branch would have higher persistence
            // and new branch name is different from existing name.
            // If so, overwrite the existing branch.

            if parent_name != &parent_branch.name {
                if self.repo.name_persistence(parent_name) > parent_branch.persistence {
                    self.terminate_branch_at(parent_branch, *parent);
                    self.new_branch(*parent, parent_name);
                }
            }
        }
    }

    /// Create a new branch in the graph and link it with a commit.
    /// Put it in the queue
    fn new_branch (&mut self, commit: Oid, name: &str)  {
        let pers = self.repo.name_persistence(name);
        let branch_id = self.graph.branch.len();
        self.graph.branch.push(BranchTrace::new(name.to_string(), commit, pers));
        self.graph.commit_branch.insert(commit, branch_id);
        self.queue.push(commit);
    }

    /// Terminate a branch at a given commit
    fn terminate_branch_at(&mut self, branch: &BranchTrace, commit: Oid) {
        panic!("not implemented");
    }
}

impl BranchTrace {
    /// Create an open branch that request a single commit
    pub fn new(name: String, commit: Oid, persistence: Pers) -> Self {
        Self {
            name,
            target: commit,
            source: commit,
            next_source: Some(commit),
            persistence,
        }
    }
}