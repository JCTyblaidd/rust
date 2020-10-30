//use smallvec::SmallVec;
use rustc_middle::mir::*;
use rustc_middle::ty::TyCtxt;
use rustc_index::bit_set::{BitMatrix, BitSet};
//use rustc_data_structures::graph;
//use std::iter;

use crate::transform::MirPass;
use crate::dataflow::impls::{MaybeLiveLocals, MaybeBorrowedLocals};
use crate::dataflow::{Analysis, ResultsCursor};


pub struct AddLifetimes;

impl<'tcx> MirPass<'tcx> for AddLifetimes {
    fn run_pass(&self, tcx: TyCtxt<'tcx>, body: &mut Body<'tcx>) {
        let body_ref = &*body;

        // The union of these two analysis is the
        //  set for which the storage is required.
        let live = MaybeLiveLocals
            .into_engine(tcx, body_ref)
            .pass_name("add_lifetimes")
            .iterate_to_fixpoint()
            .into_results_cursor(body_ref);
        let borrows = MaybeBorrowedLocals::all_borrows()
            .into_engine(tcx, body_ref)
            .pass_name("generator")
            .iterate_to_fixpoint()
            .into_results_cursor(body_ref);

        // Now use the results to calculate three bit-matrices
        //  - if a local requires storage at the start of a block
        //  - if a local requires storage after the block has terminated
        //  - if a local requires storage anywhere in a block
        let mut local_use_matrix = LocalUseMatrix::from_cursors(
            body_ref, &mut live, &mut borrows
        );

        // Now cast block usage, forward and backwards to ensure that all blocks
        //  that may require `StorageLive` & `StorageDead` are marked as used
        //  as well. This handles cases where a block that uses a local
        //   is only used by the previous block,
        local_use_matrix.propagate_usage(body_ref);

        // Now calculate the dominators and post-dominators of all blocks
        //  in the body.
        let block_dominator = body_ref.dominators();
        let block_post_dominator = body_ref.post_dominators();

        //For each block in which `StorageLive` could potentially be inserted
        // intersect the dominator, this will always suceed since the start
        // node will always be a dominator of all nodes.
        //Similarly for each block in which `StorageDead` could potentially be
        // interserted, the unwind & non-unwind cases are handled separately
        // due to the terminators not having a common dominator.
        //FIXME: this will break if the two Return split optimization occurs.










        // Any block that uses a local, but does not use it at the start, should
        //  have a `StorageLive` inserted. Similarly any block that uses a local,
        //  but does not use it at the terminator, should have a `StorageDead`
        //  inserted, (FIXME: cleanup block semantics?);
        let (
            blocks, locals
        ) = body.basic_blocks_and_local_decls_mut();
        let storage_lives = rustc_index::vec::IndexVec::from_elem(
            Vec::new(), &locals
        ); //FIXME: use smallvec[;N], and do research for a good value of N, or union_merge structure??
        let storage_deads = rustc_index::vec::IndexVec::from_elem(
            Vec::new(), &locals
        ); //FIXME: same as above, is Vec::new() the best data-structure?
        for (block, basic_block) in blocks.iter_enumerated() {
            for local in local_use_matrix.storage_block.iter(block) {
                if !local_use_matrix.storage_start.contains(block, local) {
                    storage_lives[local].push(block);
                    //FIXME: write `StorageLive`
                    local_use_matrix.write_storage_live(
                        local, basic_block, &mut live, &mut borrows
                    );
                }
                if !local_use_matrix.storage_end.contains(block, local) {
                    storage_deads[local].push(block);
                    //FIXME: write `StorageDead`, FIXME: intersction with cleanup blocks??
                }
            }
        }

        //ALT: utilize the dominator for `StorageLive`
        //    and utilize the post-dominator for `StorageDead`, _
        // leave the more complicated form to test later

        // However, in some cases it might be possible for a local to be marked with two or more
        //  `StorageLive`, `StorageDead` pairs. This should be detected and the local should
        //  be split into two or more locals instead.
        for (local, starts) in storage_lives.iter_enumerated() {
            //If a two starts can reach a common block, then they are the same
            //FIXME: record `StorageDead` as well?
            //visited[block] = colour
            // union_merge[colour]  -> 1 depth first search from each start
            // then a transform pass on the two `BasicBlock`s
            //If however, the two data-usage graphs are strictly distinct, then 
        }
        //FIXME: HOW TO???
        // - idea colour the graph or similar??, do so earlier??
        // - effectively apply data-flow, and detect that a local is used at least once

        //Find the set of start nodes
        // {start: idx}
        // depth-first-search from all starts for the given local
        // apply union-merge if they eventually intersect 




        //BUG: this idea can split locals into two `StorageLive`/Dead pairs, this
        //  should be detected & the local split into two locals instead!

        let (
            blocks, locals
        ) = body.basic_blocks_and_local_decls_mut();
        let mut tested_block = BitSet::new_empty(blocks.len());

        //For block in block IF: block is live at start, then mark all predecessors as live
        ///          if block is live at end, then mark all sucessors as live.

        //For block in block IF: block uses local & !used at start -> insert `StorageLive`
        //   IF: block uses local &!used at end -> insert `StorageDead`

        //FIXME: old borked algorithm
        for(local, _) in locals.iter_enumerated() {
            //Insert `StorageLive`
            tested_block.clear();
            for (block, block_contents) in blocks.iter_enumerated_mut() {
                if local_use_matrix.storage_start.contains(block, local) {
                    for pred in [block].iter().copied() { //FIXME: placeholder
                        if tested_block.insert(pred) {
                            if !local_use_matrix.storage_start.contains(tested_block, local);
                        }
                    }
                }
            }

            //Insert `StorageDead`
            tested_block.clear();
            for (Block, block_contents) in blocks.iter_enumerated_mut() {
                if local_use_matrix.storage_end.contains(block, local) {

                }else{

                }
            }
        }

        // apply to the graph, selective filter of edges
        // A      |->E   where {A,C,E} use nodes
        // |-->C--|        we need {A,B} do appear for `StartNodes` & {E,F} for `EndNodes`
        // B      |->F     {so start-node: trim{alive(END)}} | {end-node: trim{alive(START)}}
        //                 -> then perform dominators/post-dominator analysis? (FIXME: no need, mearly find start or end edges)

        //For each local calculate the graph:
        // {node : node in body | node in storage_block[local]}
        // {edge[u -> v] : edge in body | u in storage_end[local] & v in storage_start[local]}


        //For each local calculate the dominator & least common post-dominator of the graph
        // then find a candidate location to insert the `StorageLive` & `StorageDead` entries.

        //For each local
        for (local, local_decl) in body.local_decls().iter_enumerated() {
            //Then find the set of basic-blocks & edges
            let _ = (local, local_decl);
            //Find the post-dominator & insert `StorageDead`
        }

    }
}

/// Local usage for each basic-block
struct LocalUseMatrix {
    storage_start: BitMatrix<BasicBlock, Local>,
    storage_end: BitMatrix<BasicBlock, Local>,
    storage_block: BitMatrix<BasicBlock, Local>,
}

impl LocalUseMatrix {
    pub fn from_cursors<'mir,'tcx>(
        body_ref: &Body<'tcx>,
        live: &mut ResultsCursor<'mir, 'tcx, MaybeLiveLocals>,
        borrow: &mut ResultsCursor<'mir, 'tcx, MaybeBorrowedLocals>
    ) -> Self {
        let mut res = LocalUseMatrix {
            storage_start: BitMatrix::new(
                body_ref.basic_blocks().len(), body_ref.local_decls().len()
            ),
            storage_end: BitMatrix::new(
                body_ref.basic_blocks().len(), body_ref.local_decls().len()
            ),
            storage_block: BitMatrix::new(
                body_ref.basic_blocks().len(), body_ref.local_decls().len()
            ),
        };
        for (block, _) in body_ref.basic_blocks().iter_enumerated() {
            let terminator = body_ref.terminator_loc(block).statement_index;

            //FIXME: should storage_end be union of `BLOCK_END` or union with `TERMINATOR` also??

            // Update bits for borrow, advance foward to reduce work
            borrow.seek_to_block_start(block);
            res.storage_start.union_row_with(borrow.get(), block);
            res.storage_block.union_row_with(borrow.get(), block);
            for index in 0..terminator {
                borrow.seek_after_primary_effect(Location { block, statement_index: index });
                res.storage_block.union_row_with(borrow.get(), block);
            }
            borrow.seek_after_primary_effect(Location { block, statement_index: terminator});
            res.storage_block.union_row_with(borrow.get(), block);
            res.storage_end.union_row_with(borrow.get(), block);


            // Update bits for live, advance backward to reduce work
            live.seek_to_block_end(block);
            res.storage_end.union_row_with(live.get(), block);
            for index in (0..terminator).rev() {
                live.seek_after_primary_effect(target);
                res.storage_block.union_row_with(live.get(), block);
            }
            live.seek_to_block_start(block);
            res.storage_block.union_row_with(live.get(), block);
            res.storage_start.union_row_with(live.get(), block);
        }
        res
    }
}

/*


/// Graph that only consideres basic-block edges that
///  a control-flow dependency appears accross.
struct LocalUsageGraph<'a, 'tcx> {

    /// The set of local usages.
    body: &'a mut Body<'tcx>,

    /// The set of return blocks,
    return_blocks: SmallVec<[BasicBlock; 4]>,

    /// Virtual return block, should not exist.
    virtual_return_block: BasicBlock,

    /// The set of locals used at the end of the block,
    local_usage_start: BitMatrix<Local, BasicBlock>,

    /// The set of locals used at the terminator of the block
    local_usage_terminator: BitMatrix<Local, BasicBlock>,

    /// The current local to calculate the dominator of.
    local_query: Local,
}
impl<'a, 'tcx> graph::DirectedGraph for LocalUsageGraph<'a, 'tcx> {
    type Node = BasicBlock;
}
impl<'a, 'tcx> graph::WithStartNode for LocalUsageGraph<'a, 'tcx> {
    fn start_node(&self) -> BasicBlock {
        let limit = self.body.basic_blocks().next_index();
        assert!(limit <= self.virtual_return_block, "Invalid virtual start block");
        self.virtual_return_block
    }
}
impl<'a, 'tcx> graph::WithPredecessors for LocalUsageGraph<'a, 'tcx> {
    fn predecessors(&self, basic_block: BasicBlock) -> <Self as graph::GraphPredecessors<'_>>::Iter {
        let show_edge = self.local_usage_start.contains(self.local_query, basic_block);
        let results = if basic_block == self.virtual_return_block {
            SmallVec::new().into_iter()
        }else if show_edge {
            <Body<'tcx> as graph::WithPredecessors>::predecessors(&self.body, basic_block)
        }else{
            self.return_blocks.clone().into_iter()
        };
        results.filter(FilterPredecessor(&self))
    }
}
impl<'a, 'tcx, 'graph> graph::GraphPredecessors<'graph> for LocalUsageGraph<'a, 'tcx> {
    type Item = BasicBlock;
    type Iter = iter::Filter<<Body<'tcx> as graph::GraphPredecessors<'graph>>::Iter, FilterPredecessor<'a, 'a, 'tcx>>;
}
impl<'a, 'tcx> graph::WithSuccessors for LocalUsageGraph<'a, 'tcx> {
    fn successors(&self, basic_block: BasicBlock) -> <Self as graph::GraphSuccessors<'_>>::Iter {
        let show_edge = self.local_usage_start.contains(self.local_query, basic_block);
        let results = if basic_block == self.virtual_return_block {
            None.into_iter().chain([].iter()).cloned()
        }else if show_edge {
            <Body<'tcx> as graph::WithSuccessors>::successors(&self.body, basic_block)
        }else{
            None.into_iter().chain([].iter()).cloned()
        };
        results.filter(FilterSuccessor(&self))
    }
}
impl<'a, 'tcx, 'graph> graph::GraphSuccessors<'graph> for LocalUsageGraph<'a, 'tcx> {
    type Item = BasicBlock;
    type Iter = iter::Filter<<Body<'tcx> as graph::GraphSuccessors<'graph>>::Iter, FilterSuccessor<'a, 'a, 'tcx>>;
}
impl<'a, 'tcx> graph::WithNumNodes for LocalUsageGraph<'a, 'tcx> {
    fn num_nodes(&self) -> usize {
        self.body.basic_blocks().len() + 1
    }
}


/// Filter all incoming edges to a block, ensuring that they are a valid
///  outgoing edge on the target.
pub struct FilterPredecessor<'a, 'b, 'tcx>(&'a LocalUsageGraph<'b, 'tcx>);
impl<'a, 'b, 'tcx> FnOnce<(&BasicBlock,)> for FilterPredecessor<'a, 'b, 'tcx> {
    type Output = bool;
    extern "rust-call" fn call_once(self, (block,): (&BasicBlock,)) -> bool {
        self.0.local_usage_terminator.contains(self.0.local_query, *block)
    }
}
impl<'a, 'b, 'tcx> FnMut<(&BasicBlock,)> for FilterPredecessor<'a, 'b, 'tcx> {
    extern "rust-call" fn call_mut(&mut self, (block,): (&BasicBlock,)) -> bool {
        self.0.local_usage_terminator.contains(self.0.local_query, *block)
    }
}
impl<'a, 'b, 'tcx> Fn<(&BasicBlock,)> for FilterPredecessor<'a, 'b, 'tcx> {
    extern "rust-call" fn call(&self, (block,): (&BasicBlock,)) -> bool {
        self.0.local_usage_terminator.contains(self.0.local_query, *block)
    }
}

/// Filter all outgoing edges to a block, ensuring that they are a valid
///   incoming edge to the target.
pub struct FilterSuccessor<'a, 'b, 'tcx>(&'a LocalUsageGraph<'b, 'tcx>);
impl<'a, 'b, 'tcx> FnOnce<(&BasicBlock,)> for FilterSuccessor<'a, 'b, 'tcx> {
    type Output = bool;
    extern "rust-call" fn call_once(self, (block,): (&BasicBlock,)) -> bool {
        self.0.local_usage_start.contains(self.0.local_query, *block)
    }
}
impl<'a, 'b, 'tcx> FnMut<(&BasicBlock,)> for FilterSuccessor<'a, 'b, 'tcx> {
    extern "rust-call" fn call_mut(&mut self, (block,): (&BasicBlock,)) -> bool {
        self.0.local_usage_start.contains(self.0.local_query, *block)
    }
}
impl<'a, 'b, 'tcx> Fn<(&BasicBlock,)> for FilterSuccessor<'a, 'b, 'tcx> {
    extern "rust-call" fn call(&self, (block,): (&BasicBlock,)) -> bool {
        self.0.local_usage_start.contains(self.0.local_query, *block)
    }
}


*/