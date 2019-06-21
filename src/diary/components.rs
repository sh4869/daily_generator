use maud::{html, Markup, PreEscaped, DOCTYPE};

const HEADER_WORD: &str = "You will understand if you come here, You'll overlook your sleepiness";

const CSSLIST: [&str; 5] = [
    "https://cdnjs.cloudflare.com/ajax/libs/normalize/7.0.0/normalize.css",
    "/static/css/index.css",
    "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/styles/hopscotch.min.css",
    "https://fonts.googleapis.com/css?family=Noto+Sans+JP",
    "https://cdnjs.cloudflare.com/ajax/libs/flexboxgrid/6.3.1/flexboxgrid.min.css",
];

const HIGHLIGHT_JS: &str = r##"<script src="//cdnjs.cloudflare.com/ajax/libs/highlight.js/9.12.0/highlight.min.js"></script><script>hljs.initHighlightingOnLoad();</script>"##;

fn header() -> Markup {
    html! {
        div.row {
            div#header class=("col-xs-12 top") {
                a href=("/") {
                    h1.title {"Daily Bread"}
                }
                p {(HEADER_WORD)}
            }
        }
    }
}

fn footer() -> Markup {
    html! {
        div.row {
            div#footer class=("col-xs-12") {
                footer {
                    p {(PreEscaped("&copy; 2017-2019 <a href=\"http://sh4869.net\">sh4869</a>") )}
                }
            }
        }
    }
}

pub fn page(title: &str, page: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="ja" {
            head {
                meta chaset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                @for url in &CSSLIST {
                    link rel="stylesheet" href=(url);
                }
                (PreEscaped(HIGHLIGHT_JS))
                title {(title)}
            }
            body {
                (header())
                (page)
                (footer())
            }
        }
    }
}