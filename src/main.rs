//! SigmaOS-oh-my-zsh — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_oh_my_zsh_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-oh-my-zsh: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-oh-my-zsh: ok");
}
