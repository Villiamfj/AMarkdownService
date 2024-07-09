use pulldown_cmark::html as other_html;
use pulldown_cmark::{Options, Parser};
use yew::html;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::ServerRenderer;

#[derive(Default, Properties, Clone, PartialEq)]
pub struct Probs {
    markdown: String,
    pages: Vec<String>,
}

/// Reads css from file and returns it as a VNode
fn get_style_from_file(path: String) -> VNode {
    // reading file
    let mut file = std::fs::read_to_string(path).unwrap();
    // wrapping in style tags
    file.insert_str(0, "<style>");
    file.push_str("</style>");

    //converting to VNode (element in dom)
    VNode::from_html_unchecked(AttrValue::from(file))
}
/// Component to be rendered with content from markdown
#[function_component]
fn MarkdownApp(props: &Probs) -> Html {
    let Probs { markdown, pages } = props;
    let mark = VNode::from_html_unchecked(AttrValue::from(markdown.clone()));
    let style = get_style_from_file("main.css".to_string());

    html! {
        <root>
            {style}
            <meta name="viewport" content="width=device-width, initial-scale=1"/>
            <ul class="navbar">
                {pages.iter().map(|page| {
                        let name = remove_md(page.clone());
                        html! {
                            <li class="nav">
                                <a href={name.clone()}>{name}</a>
                            </li>
                        }
                    }).collect::<Html>()
                }
            </ul>
            <div class="content">{mark}</div>
        </root>
    }
}

fn remove_md(path: String) -> String {
    let mut path = path;
    if path.ends_with(".md") {
        path.pop();
        path.pop();
        path.pop();
    }
    path
}

pub async fn render_markdown(mark: String, pages: Vec<String>) -> String {
    let parsed_html = parse_markdown(mark);

    let renderer = ServerRenderer::<MarkdownApp>::with_props(|| Probs {
        markdown: parsed_html,
        pages,
    });
    let mut rendered = String::new();
    renderer.render_to_string(&mut rendered).await;
    rendered
}

fn parse_markdown(mark: String) -> String {
    let mut options: Options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&mark, options);
    let mut html_output = String::new();
    other_html::push_html(&mut html_output, parser);
    html_output
}
