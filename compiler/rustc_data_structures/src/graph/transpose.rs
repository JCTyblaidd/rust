use super::{
    DirectedGraph, WithNumNodes, WithNumEdges,
    GraphPredecessors, GraphSuccessors, GraphNodes,
    WithPredecessors, WithSuccessors, WithNodes
};

pub struct TranposeGraph<G>(pub G);

impl<G: DirectedGraph> DirectedGraph for TranposeGraph<G> {
    type Node = G::Node;
}

impl<G: WithNumNodes> WithNumNodes for TranposeGraph<G> {
    fn num_nodes(&self) -> usize {
        self.0.num_nodes()
    }
}
impl<G: WithNumEdges> WithNumEdges for TranposeGraph<G> {
    fn num_edges(&self) -> usize {
        self.0.num_edges()
    }
}

impl<G: WithNodes> WithNodes for TranposeGraph<G>{
    fn nodes(&self) -> <Self as GraphNodes<'_>>::Iter {
        self.0.nodes()
    }
}
impl<'graph, G: WithNodes> GraphNodes<'graph> for TranposeGraph<G> {
    type Item = G::Item;
    type Iter =  <G as GraphNodes<'graph>>::Iter;
}

impl<G: WithPredecessors> WithPredecessors for TranposeGraph<G> {
    fn predecessors(&self) -> <Self as GraphPredecessors<'_>>::Iter {
        self.0.predecessors(node)
    }
    fn has_predecessors(&self, node: Self::Node) -> bool {
        self.0.has_predecessors(node)
    }
}
impl<'graph, G: WithPredecessors> GraphNodes<'graph> for TranposeGraph<G> {
    type Item = G::Item;
    type Iter =  <G as GraphPredecessors<'graph>>::Iter;
}

impl<G: WithSuccessors> WithSuccessors for TranposeGraph<G>{
    fn successors(&self) -> <Self as GraphNodes<'_>>::Iter {
        self.0.successors()
    }
    fn has_sucessors(&self, node: Self::Node) -> bool {
        self.0.has_sucessors(node)
    }
}
impl<'graph, G: WithSuccessors> GraphSuccessors<'graph> for TranposeGraph<G> {
    type Item = G::Item;
    type Iter =  <G as GraphSuccessors<'graph>>::Iter;
}