fn main() {
    //OUT_DIRはbuild.rsが無いとうまく働かない。
    println!("Hello, world!, {}", concat!(env!("OUT_DIR"), "\\windows.rs"));
}