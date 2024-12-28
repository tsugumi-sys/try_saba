//! This is a DOM Window object.
//! https://html.spec.whatwg.org/multipage/nav-history-apis.html#window

use crate::renderer::dom::node::Node;
use crate::renderer::dom::node::NodeKind;
use alloc::rc::Rc;
use core::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Window {
    document: Rc<RefCell<Node>>,
}

impl Window {
    pub fn new() -> Self {
        let window = Self {
            document: Rc::new(RefCell::new(Node::new(NodeKind::Document))),
        };

        window
            .document
            .borrow_mut()
            .set_window(Rc::downgrade(&Rc::new(RefCell::new(window.clone()))));
        window
    }

    pub fn document(&self) -> Rc<RefCell<Node>> {
        self.document.clone()
    }
}
