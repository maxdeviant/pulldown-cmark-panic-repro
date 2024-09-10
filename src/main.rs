use pulldown_cmark::Options;

fn main() {
    let options = Options::all();

    let markdown_input = "```gleam\nfn() -> a\n```\n [abc] check `foobar_raz`\n\n Some preamble `foobar_raz`, not `barfoo_raz`\n :D\n\n This should fix:\n\n &gt; Something is wrong!";
    let parser = pulldown_cmark::Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);

    println!("Parsed HTML:\n{}", html_output);
}
