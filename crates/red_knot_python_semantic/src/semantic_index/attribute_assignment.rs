use crate::semantic_index::expression::Expression;
use ruff_db::files::File;

#[salsa::tracked]
pub(crate) struct AttributeAssignment<'db> {
    /// The file in which the attribute assignment occurs.
    #[id]
    pub(crate) file: File,

    /// The type annotation of the assignment.
    pub(crate) annotation: Expression<'db>,

    #[no_eq]
    count: countme::Count<AttributeAssignment<'static>>,
}
