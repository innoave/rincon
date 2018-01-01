
use ast::NodeId;

#[allow(missing_copy_implementations)]
#[derive(Debug, PartialEq)]
pub struct IdGen {
    next: NodeId,
}

impl IdGen {
    pub fn new() -> Self {
        IdGen {
            next: 0,
        }
    }

    pub fn next(&mut self) -> NodeId {
        let id = self.next;
        self.next = id + 1;
        id
    }
}
