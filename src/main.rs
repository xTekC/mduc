
fn version() -> String {
    String::from("0.1.0")
}

fn info() {
    let v = version();
    println!("markdup, a markdown to html compiler, written by xTeKc");
    println!("version {}", v);
}

fn main() {
    info();
}
