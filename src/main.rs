/*
 * main.rs - a program that compiles markdown in ../markdown/ into html stored under ../compiled/
 * Copyright (C) 2025 pastaya
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Result, Watcher};
use pulldown_cmark::{Options, Parser, html};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone)]
struct CompilerConfig {
    input_dir: PathBuf,
    output_dir: PathBuf,
}

// options for pulldown_cmark
const OPTIONS: Options = Options::ENABLE_STRIKETHROUGH
    .union(Options::ENABLE_FOOTNOTES)
    .union(Options::ENABLE_SMART_PUNCTUATION);

impl CompilerConfig {
    fn new(input: &str, output: &str) -> Self {
        Self {
            input_dir: PathBuf::from(input),
            output_dir: PathBuf::from(output),
        }
    }
}

// seems obvious from the name right?
fn compile_md_to_html(path: &Path, config: &CompilerConfig) -> Result<()> {
    if path.extension().map(|e| e == "md").unwrap_or(false) {
        let content = fs::read_to_string(path)?;
        let parser = Parser::new_ext(&content, OPTIONS);
        let mut body = String::new();

        html::push_html(&mut body, parser);

        // this basically injects extra html, right now its just includes the css file and syntax
        // highlighting

        let full_html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">

    <title>{title}</title>

    <link rel="stylesheet" href="/blog/style.css">

    <link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.11.1/build/styles/default.min.css">

    <script src="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.11.1/build/highlight.min.js"></script>

    <script>hljs.highlightAll();</script>
    <!-- highlight.js provides syntax highlighting -->
</head>
<body>
    {body}
</body>
</html>"#,
            title = path.file_stem().unwrap().to_string_lossy(),
            body = body
        );

        let mut output_path = config.output_dir.join(path.file_stem().unwrap());
        output_path.set_extension("html");

        fs::write(&output_path, full_html)?;
        println!("Compiled {:?} → {:?}", path, output_path);
    }
    Ok(())
}

fn main() -> Result<()> {
    let config = CompilerConfig::new("./markdown", "./compiled");

    fs::create_dir_all(&config.output_dir)?;

    let mut watcher = RecommendedWatcher::new(
        {
            let config = config.clone(); // probably unidiomatic but it works
            move |res: Result<Event>| match res {
                Ok(event) => {
                    if matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_)) {
                        for path in event.paths {
                            let _ = compile_md_to_html(&path, &config); // SO MUCH BOILERPLATE
                        }
                    }
                }
                Err(e) => eprintln!("watch go ka BOOM!: {:?}", e),
            }
        },
        Config::default(),
    )?;

    watcher.watch(&config.input_dir, RecursiveMode::Recursive)?;
    println!(
        "i am watching {:?} for .md files... very nsa like",
        config.input_dir
    );

    Ok(())
}
