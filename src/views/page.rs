use hypertext::prelude::*;

/// Generates the HTML structure for a page with a title and content
#[component]
pub fn page<'a, R: Renderable>(title: &'a str, children: &R) -> impl Renderable {
    maud! {
        !DOCTYPE
        html {
            head {
                title { (title) }
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                // TODO for now unpkg is used, but this should be replaced with a local copy in the future
                script src="https://unpkg.com/htmx.org" {}
                // TODO probably figure out a way clean way to use the tailwnindcss cli to also decrease the bundle size
                //script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4" {}
                link rel="stylesheet" href="/static/styles.css";
            }
            body class="bg-gray-900 text-gray-100" { (children) }
        }
    }
}
