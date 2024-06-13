use leptos::html::Input;
use leptos::{spawn_local, NodeRef};
use std::time::Duration;

pub fn set_focus(node: NodeRef<Input>) -> () {
    spawn_local(async move {
        // Allow some time for the element to be rendered
        leptos::set_timeout(
            move || {
                if let Some(node) = node.get() {
                    node.focus().unwrap();
                }
            },
            Duration::new(0, 0),
        );
    });
}
