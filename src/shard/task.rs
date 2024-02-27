use super::ast::Ast;
use crate::path::Path;
/// A task
pub struct Task {
    /// Source of the task
    src: Path,
    /// Content
    item: Ast,
    /// Sub-tasks
    subtasks: Vec<Task>,
}
