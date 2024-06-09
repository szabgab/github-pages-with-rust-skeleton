// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     std::fs::create_dir_all("_site").unwrap();
//     let html = r#"
//         <h1>Hello from Rust</h1>
//     "#;
//     std::fs::write("_site/index.html", html)?;
//     Ok(())
// }

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     std::fs::create_dir_all("_site").unwrap();

//     let markdown = std::fs::read_to_string("README.md")?;
//     match markdown2html(&markdown) {
//         Ok(html) => std::fs::write("_site/index.html", html)?,
//         Err(err) => return Err(Box::<dyn std::error::Error>::from(format!("{err}"))),
//     }

//     Ok(())
// }

fn markdown2html(content: &str) -> Result<String, markdown::message::Message> {
    let html = markdown::to_html_with_options(
        content,
        &markdown::Options {
            compile: markdown::CompileOptions {
                allow_dangerous_html: true,
                ..markdown::CompileOptions::default()
            },
            ..markdown::Options::gfm()
        },
    )?;

    Ok(html)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("_site").unwrap();

    let markdown = std::fs::read_to_string("README.md")?;
    match markdown2html(&markdown) {
        Ok(html) => {
            let template = include_str!("../templates/page.html");
            let template = liquid::ParserBuilder::with_stdlib()
                .build()
                .unwrap()
                .parse(template)
                .unwrap();

            let globals = liquid::object!({
                "content": &html,
                "title": "Rust Maven demo",
            });
            let output = template.render(&globals).unwrap();

            std::fs::write("_site/index.html", output)?;
        }
        Err(err) => return Err(Box::<dyn std::error::Error>::from(format!("{err}"))),
    }

    Ok(())
}
