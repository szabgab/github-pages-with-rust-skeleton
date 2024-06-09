use std::fs::create_dir_all;

fn main() {
    create_dir_all("_site").unwrap();
    let html = r#"
        <h1>Hello World</h1>
    "#;
    std::fs::write("_site/index.html", html).unwrap();

}
