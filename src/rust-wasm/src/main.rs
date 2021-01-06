mod lib;

fn main() {
  let markdown_input = "Hello world, this is a ~~complicated~~ *very simple* example.";
  let expected_html = "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";

  let html_output = lib::render_md(markdown_input);
  assert_eq!(expected_html, &html_output);
}