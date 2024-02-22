use super::ast;
use crate::path::Path;
use markdown::mdast::ListItem;

/// A task
pub struct Task {
    /// Source of the task
    src: Path,
    /// Content
    item: ListItem,
    /// Sub-tasks
    subtasks: Vec<Task>,
}

/// Walk the tree and get all the tasks.
pub fn iter<'a>(src: &'a Path, tree: &'a ast::Node) -> impl Iterator<Item = Task> + 'a {
    ast::iter_tree_with_stop(tree, |node| ast::is_checkable_item(node)).map(|(node, _)| {
        let item = ast::expect_list_item(node).to_owned();
        Task::new(src, item)
    })
}

impl Task {
    pub fn new(src: &Path, item: ast::ListItem) -> Self {
        let subtasks = item.children.iter().map(|c| iter(src, c)).flat_map(|c| c);

        Self {
            src: src.to_owned(),
            item: item.clone(),
            subtasks: subtasks.collect(),
        }
    }
}
