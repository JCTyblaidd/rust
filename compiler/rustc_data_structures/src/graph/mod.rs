use rustc_index::vec::Idx;

pub mod dominators;
pub mod implementation;
pub mod iterate;
mod reference;
pub mod transpose;
pub mod scc;
pub mod vec_graph;

#[cfg(test)]
mod tests;

pub trait DirectedGraph {
    type Node: Idx;
}

pub trait WithNumNodes: DirectedGraph {
    fn num_nodes(&self) -> usize;
}

pub trait WithNumEdges: DirectedGraph {
    fn num_edges(&self) -> usize;
}


pub trait WithNodes: DirectedGraph + WithNumNodes
where
    Self: for<'graph> GraphNodes<'graph, Item = <Self as DirectedGraph>::Node>,
{
    fn nodes(&self) -> <Self as GraphNodes<'_>>::Iter;
}

#[allow(unused_lifetimes)]
pub trait GraphNodes<'graph> {
    type Item;
    type Iter: Iterator<Item = Self::Item>;
}

pub trait WithSuccessors: DirectedGraph
where
    Self: for<'graph> GraphSuccessors<'graph, Item = <Self as DirectedGraph>::Node>,
{
    fn successors(&self, node: Self::Node) -> <Self as GraphSuccessors<'_>>::Iter;

    fn has_sucessors(&self, node: Self::Node) -> bool {
        self.successors(node).next().is_some()
    }

    fn depth_first_search(&self, from: Self::Node) -> iterate::DepthFirstSearch<'_, Self>
    where
        Self: WithNumNodes,
    {
        iterate::DepthFirstSearch::new(self, from)
    }
}

#[allow(unused_lifetimes)]
pub trait GraphSuccessors<'graph> {
    type Item;
    type Iter: Iterator<Item = Self::Item>;
}

pub trait WithPredecessors: DirectedGraph
where
    Self: for<'graph> GraphPredecessors<'graph, Item = <Self as DirectedGraph>::Node>,
{
    fn predecessors(&self, node: Self::Node) -> <Self as GraphPredecessors<'_>>::Iter;

    fn has_predecessors(&self, node: Self::Node) -> bool {
        self.predecessors(node).next().is_some()
    }
}

#[allow(unused_lifetimes)]
pub trait GraphPredecessors<'graph> {
    type Item;
    type Iter: Iterator<Item = Self::Item>;
}

pub trait WithStartNode: DirectedGraph {
    fn start_node(&self) -> Self::Node;
}

pub trait ControlFlowGraph:
    DirectedGraph + WithStartNode + WithPredecessors + WithSuccessors + WithNumNodes
{
    // convenient trait
}

impl<T> ControlFlowGraph for T where
    T: DirectedGraph + WithStartNode + WithPredecessors + WithSuccessors + WithNumNodes
{
}

/// Returns `true` if the graph has a cycle that is reachable from the start node.
pub fn is_cyclic<G>(graph: &G) -> bool
where
    G: ?Sized + DirectedGraph + WithStartNode + WithSuccessors + WithNumNodes,
{
    iterate::TriColorDepthFirstSearch::new(graph)
        .run_from_start(&mut iterate::CycleDetector)
        .is_some()
}
