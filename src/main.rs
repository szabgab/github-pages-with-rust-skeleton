fn main() {
    std::fs::create_dir_all("_site").unwrap();
    let html = r#"
        <h1>Hello from Rust</h1>
    "#;
    std::fs::write("_site/index.html", html).unwrap();
}
