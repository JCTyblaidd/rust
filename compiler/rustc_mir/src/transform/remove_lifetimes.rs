//! Find and remove any and all `StorageDead` and `StorageLive` statements,
//!  all `StorageLive` statements are replaced with no-ops
//!  all `StorageDead` statements are replaced with `InvalidateBorrows`

use crate::transform::MirPass;
use rustc_middle::mir::visit::MutVisitor;
use rustc_middle::mir::*;
use rustc_middle::ty::TyCtxt;

/// Remove all lifetime markers and replace them all with `InvalidateBorrows`
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
            StatementKind::StorageLive(_) => {
                statement.make_nop();
            }
            StatementKind::StorageDead(local) => {
                statement.kind = StatementKind::InvalidateBorrows(*local);
            }
            _ => (),
        }
    }
}
