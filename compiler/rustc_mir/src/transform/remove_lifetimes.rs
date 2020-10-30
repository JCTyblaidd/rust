//! Remove and replace lifetime markers with alternatives
//!  that can be placed multiple times, for efficient MIR opt.
//! Converts `StorageLive` to `MakeUnitialized`
//!      and `StorageDead` to `InvalidateBorrows`

use crate::transform::MirPass;
use rustc_middle::mir::visit::MutVisitor;
use rustc_middle::mir::*;
use rustc_middle::ty::TyCtxt;

/// Remove `StorageDead` markers,
pub struct RemoveLifetimes;

impl<'tcx> MirPass<'tcx> for RemoveLifetimes {
    fn run_pass(&self, tcx: TyCtxt<'tcx>, body: &mut Body<'tcx>) {
        LifetimeRemoveVisitor { tcx }.visit_body(body);
    }
}

struct LifetimeRemoveVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
}

impl<'tcx> MutVisitor<'tcx> for LifetimeRemoveVisitor<'tcx> {
    fn tcx(&self) -> TyCtxt<'tcx> {
        self.tcx
    }
    fn visit_statement(&mut self, statement: &mut Statement<'tcx>, _location: Location) {
        match &statement.kind {
            StatementKind::StorageLive(local) => {
                statement.kind = StatementKind::MarkUninitialized(*local);
            },
            StatementKind::StorageDead(local) => {
                statement.kind = StatementKind::InvalidateBorrows(*local);
            },
            _ => {}
        }
    }
}
