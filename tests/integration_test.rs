//! Integration tests for SigmaOS-oh-my-zsh.

use sigmaos_oh_my_zsh_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
