



fn info() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let description = env!("CARGO_PKG_DESCRIPTION");
    let author = env!("CARGO_PKG_AUTHORS");
    println!("{}, {}. Written by {}", name, description, author);
    println!("Version {}", version);
}

fn main() {
    info();
}
