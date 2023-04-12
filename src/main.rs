#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_desktop::{
    tao::menu::{MenuBar, MenuItem},
    Config, LogicalSize, WindowBuilder,
};
// use dioxus_router::{Link, Route, Router};

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
                <!-- Hightlight JS -->
                <link rel="stylesheet" href="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.7.0/build/styles/default.min.css">
                <script src="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.7.0/build/highlight.min.js"></script>
                <!-- and it's easy to individually load additional languages -->
                <!-- <script src="https://cdn.jsdelivr.net/gh/highlightjs/cdn-release@11.7.0/build/languages/go.min.js"></script> -->
                <script>hljs.highlightAll();</script>


                <!-- markdown -->
                <script src="https://cdn.jsdelivr.net/npm/marked/marked.min.js"></script>
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
    let dialogs: Vec<ChatGPT::Dialog> = vec![];
    let ids = ChatGPT::DialogID {
        current_id: 0,
        next_id: 1,
    };
    use_shared_state_provider(cx, || dialogs);
    use_shared_state_provider(cx, || ids);

    cx.render(rsx! {
        div {
            class: "chat_container",
            style { include_str!("./assests/style.css")},
            script {include_str!("../src/assests/codeblockcopy.js")}
            div {
                class: "chat_column_left",
                ChatGPT::Dialog(cx)
            }
            div {
                class: "chat_column_right",
                ChatGPT::Message(cx),
                ChatGPT::Input(cx)
            }
        }
    })
}

mod ChatGPT {

    // Dialogs data struct
    #[derive(Debug, Clone)]
    pub struct Dialog {
        pub id: i32,
        pub name: String,
        Message: Vec<Message>,
    }

    // Dialogs IDs data struct
    #[derive(Debug)]
    pub struct DialogID {
        pub current_id: i32,
        pub next_id: i32,
    }

    // Roles enum
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Role {
        User,
        Assistant,
    }

    // Messages data struct
    #[derive(Debug, Clone)]
    pub struct Message {
        pub id: i32,
        pub user: Role,
        pub content: String,
        pub time: String,
    }

    #[derive(PartialEq, Props)]
    pub struct RenderMessageProp {
        content: String,
        id: i32,
    }

    use dioxus::{
        html::input_data::keyboard_types::{Code, Modifiers},
        prelude::*,
    };

    pub fn Dialog(cx: Scope) -> Element {
        cx.render(rsx! {
            Dialog_components::Dialog_tab(cx),
            Dialog_components::Dialog_control(cx),
        })
    }

    mod Dialog_components {
        use super::*;
        pub fn Dialog_tab(cx: Scope) -> Element {
            let dialogs = use_shared_state::<Vec<Dialog>>(cx).unwrap();
            let ids = use_shared_state::<DialogID>(cx).unwrap();
            let current_id = ids.read().current_id;

            cx.render(rsx! {
                div {
                    class: "dialog_container",
                    // render dialog list
                    dialogs.read().iter().map(|dialog| {
                        let id = dialog.id;
                        rsx! {
                            div {
                                class: if id == current_id {
                                    "dialog_tab active"
                                } else {
                                    "dialog_tab"
                                },
                                onclick: move |_| {
                                    ids.write().current_id = id;
                                },
                                "{dialog.name}"
                                button {
                                    class: "dialog_tab_delete_button",
                                    onclick: move |_| {
                                        dialogs.write().retain(|dialog| dialog.id != id);
                                        ids.write().current_id = 0;
                                    },
                                    "Delete"
                                }
                            }
                        }
                    })
                }
            })
        }

        pub fn Dialog_control(cx: Scope) -> Element {
            let dialogs = use_shared_state::<Vec<Dialog>>(cx).unwrap();
            let ids = use_shared_state::<DialogID>(cx).unwrap();
            cx.render(rsx! {
                div {
                    class: "dialog_control",
                    // buttom thats add new dialog
                    button {
                        class: "control_button",
                        onclick: move |_| {
                            let new_id = ids.read().next_id;
                            dialogs.write().push(Dialog {
                                id: new_id,
                                name: format!("Dialog {}", new_id),
                                Message: vec![],
                            });
                            ids.write().current_id = new_id;
                            ids.write().next_id += 1;
                        },
                        "Add new dialog",
                    }
                    button {
                        class: "control_button",
                        "Settings"
                    }
                }
            })
        }
    }

    pub fn Message(cx: Scope) -> Element {
        let dialogs = use_shared_state::<Vec<Dialog>>(cx).unwrap();
        let ids = use_shared_state::<DialogID>(cx).unwrap();
        let current_id = ids.read().current_id;
        if current_id == 0 {
            return None;
        }
        let binding = dialogs.read();
        let current_dialog = binding
            .iter()
            .find(|dialog| dialog.id == current_id)
            .unwrap();

        cx.render(rsx! {
            div {
                class: "message_container",
                // render messages
                current_dialog.Message.iter().map(|message| {
                    let message_container_type = if message.user == Role::User {
                        "user_message_container"
                    } else {
                        "assistant_message_container"
                    };
                    rsx! {
                        div {
                            class: message_container_type,
                            RenderMessage {
                                content: message.content.clone(),
                                id: message.id,
                            }
                        }
                    }
                })
            }
        })
    }

    fn RenderMessage(cx: Scope<RenderMessageProp>) -> Element {
        cx.render(rsx! {
            div {
                id: "{cx.props.id}",
                script {
                    format!("document.getElementById('{}').innerHTML = marked.parse('{}');", cx.props.id, cx.props.content.replace("\n", "\\n").replace("'", "\\'")),
                    "hljs.highlightAll();"
                    "add_copy_tag();"
                }
                // "{cx.props.content}",
            }
        })
    }

    pub fn Input(cx: Scope) -> Element {
        let dialogs = use_shared_state::<Vec<Dialog>>(cx).unwrap();
        let ids = use_shared_state::<DialogID>(cx).unwrap();
        let current_id = ids.read().current_id;
        if current_id == 0 {
            return None;
        }

        let input_contents: &UseState<String> = use_state(cx, || "".into());

        println!("{:?}", input_contents);

        let send_message = move || {
            let mut binding = dialogs.write();
            let current_dialog = binding
                .iter_mut()
                .find(|dialog| dialog.id == current_id)
                .unwrap();
            current_dialog.Message.push(Message {
                id: rand::random::<i32>(), // todo
                user: Role::User,
                content: input_contents.get().clone(),
                time: "2020-01-01 00:00:00".to_string(),
            });
            current_dialog.Message.push(Message {
                id: rand::random::<i32>(),
                user: Role::Assistant,
                content: input_contents.get().clone(),
                time: "2020-01-01 00:00:00".to_string(),
            });
            input_contents.set(String::new());
        };

        cx.render(rsx! {
            div {
                class: "input_container",
                div {
                    "{input_contents}"
                }
                textarea {
                    value: "{input_contents}",
                    placeholder: "Prompt ...",
                    oninput: move |evt| { input_contents.set(evt.value.clone())},
                    onkeyup: move |evt| {
                        println!("{:?}",evt);
                        if evt.code() == Code::Enter && evt.modifiers() == Modifiers::SHIFT && input_contents.is_empty() == false {
                            send_message();
                        }
                    },
                    // onfocus: move |_| if input_contents.as_str().eq("Input ...") { input_contents.set(String::new()) },
                    // onblur: move |_| if input_contents.is_empty() { input_contents.set(default_input_hints.clone()) }

                }
                button {
                    onclick: move |_| {
                        println!("click send");
                        send_message();
                    },
                    "Send\n「Shift + Enter」"
                }


            }
        })
    }
}
