//! SigmaOS-archlinux-themes-lxde — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_archlinux_themes_lxde_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-archlinux-themes-lxde: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-archlinux-themes-lxde: ok");
}
