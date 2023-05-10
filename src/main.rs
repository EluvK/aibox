#![feature(async_fn_in_trait)]
#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_desktop::{
    tao::menu::{MenuBar, MenuItem},
    Config, LogicalSize, WindowBuilder,
};
// use dioxus_router::{Link, Route, Router};

mod components;
mod gpt;

fn main() {
    hot_reload_init!();
    dioxus_desktop::launch_cfg(
        AIBox,
        Config::new().with_window(
            WindowBuilder::new()
                .with_title("AI Box")
                .with_inner_size(LogicalSize::new(2400, 1200))
                .with_menu({
                    let mut menu = MenuBar::new();

                    let mut app_menu = MenuBar::new();
                    app_menu.add_native_item(MenuItem::Minimize);
                    app_menu.add_native_item(MenuItem::Quit);

                    menu.add_submenu("AI-BOX", true, app_menu);
                    menu
                }),
        ).with_custom_head(
            r#"
                <!-- now is local -->
                <!-- Hightlight JS -->
                <!-- <link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.7.0/build/styles/default.min.css"> -->
                <!-- <script src="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.7.0/build/highlight.min.js"></script> -->
                <!-- and it's easy to individually load additional languages -->
                <!-- <script src="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.7.0/build/languages/go.min.js"></script> -->
                <!-- <script>hljs.highlightAll();</script> -->
                <!-- markdown -->
                <!-- <script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script> -->
            "#.into()
        ),
    );
}

fn AIBox(cx: Scope) -> Element {
    // let app_row = r"
    //     display: flex;
    //     flex-direction: row;
    //     width: 96vw;
    //     height: 36px;
    //     text-align: center;
    //     line-height: 24px;
    //     box-sizing: border-box;
    //     border: 1px solid #eee;
    //     margin: 0 1vw;
    // ";

    // let app_logo = r"
    //     flex-grow: 1;

    //     width: 80px;
    //     min-width: 10vw;
    // ";

    // let app_tab = r"
    //     flex-grow: 3;

    //     background-color: #f6f4f0;
    //     text-decoration: none;
    //     padding: 4px;
    //     border: 1px solid #aaaaaa;
    //     color: #000;
    //     font-size: 16px;
    // ";

    cx.render(rsx! { Chat{}

        // Router {
            // div {
            //     style: app_row,
            //     div { style: app_logo, "Logo" }
            //     div { style: app_tab, Link { to: "/", "Go home!"} }
            //     div { style: app_tab, Link { to: "/blog", "Blog posts"} }
            //     div { style: app_tab, Link { to: "/chatgpt", "GPT"} }
            // }
            // Route { to: "/", Chat{} }
            // Route { to: "/blog", Blog{} }
            // Route { to: "/chatgpt", Chat{} }
            // Route { to: "", "Err 404 Route Not Found" }
        // }

    })
}

// fn Home(cx: Scope) -> Element {
//     cx.render(rsx!(
//         div {
//             "Home"
//         }
//     ))
// }

// fn Blog(cx: Scope) -> Element {
//     cx.render(rsx!(
//         div {
//             "Blog"
//         }
//     ))
// }

fn Chat(cx: Scope) -> Element {
    // Dialogs state
    // todo should load state from files.
    let dialogs: Vec<components::ChatContext> = vec![];
    let ids = components::ChatContextID {
        current_id: 0,
        next_id: 1,
    };
    use_shared_state_provider(cx, || dialogs);
    use_shared_state_provider(cx, || ids);

    cx.render(rsx! {
        div {
            class: "chat_container",
            style { include_str!("./assets/default.min.css")},
            script { include_str!("./assets/highlight.min.js")}
            script { include_str!("../src/assets/codeblockcopy.js")},
            script { include_str!("../src/assets/marked.min.js")},
            style { include_str!("./assets/style.css")},
            div {
                class: "chat_column_left",
                components::Dialog(cx)
            }
            div {
                class: "chat_column_right",
                components::Message(cx),
                components::Input(cx)
            }
        }
    })
}
