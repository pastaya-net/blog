# How I made this blog

I had this idea for a nice simple markdown based blog that WASN'T

1. too complex
2. ~~bad~~ code (well if you ask anyone good at rust it would be bad)
3. CLIENT SIDE MARKDOWN (bad SEO which is literally what I NEED right now)

So I searched through existing solutions and they ALL were either

1. not so simple
2. way too complex to setup
3. kinda BLOATED

I gave up and just realized I could make my own. And I'll do it in Rust (cuz that's the only language I know lol)  
using a "few" dependencies I cooked up this short program (also syntax highlighting test)

```rust
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
    <script src="//cdnjs.cloudflare.com/ajax/libs/highlight.js/11.7.0/highlight.min.js"></script>
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
        "i am seeing {:?} for .md files... very nsa like",
        config.input_dir
    );

    loop {} // idk how to keep apps alive lol
}
```

basically all it does is just

1. watches the markdown/ folder
2. if there are any changes in the markdown folder (new file, modifed, etc) then
3. compiles markdown into html (and adds extra boilerplate for stuff like css, title (and in the future embed support))

it took alot of time actually

## The Problems I encountered

At first I used the `markdown` crate from crates.io because it was the most simple one (it was but at a cost)  
I could've just left it at there BUT I wanted syntax highlighting.  
"why can't you use syntax highlighting and the markdown crate??"  
the markdown crate produces THIS in a code block

> `<p>example example<code>print("hello")</code>`

HOWEVER
highlight.js wants THIS

> `<pre><code>print("hello")</code></pre>`

tried my best to explain it but  
markdown can ONLY make inline code unlike pulldown_cmark which produces the output highlight.js needs  
anyways that uh ends my opening blog post

cya!

-- pastaya.
