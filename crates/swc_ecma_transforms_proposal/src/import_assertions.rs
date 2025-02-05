use swc_ecma_ast::{ExportAll, ImportDecl, NamedExport};
use swc_ecma_visit::{as_folder, noop_visit_mut_type, Fold, VisitMut};

pub fn import_assertions() -> impl VisitMut + Fold {
    as_folder(ImportAssertions)
}
struct ImportAssertions;

impl VisitMut for ImportAssertions {
    noop_visit_mut_type!();

    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        n.asserts = None;
    }

    fn visit_mut_export_all(&mut self, n: &mut ExportAll) {
        n.asserts = None;
    }

    fn visit_mut_named_export(&mut self, n: &mut NamedExport) {
        n.asserts = None;
    }
}
