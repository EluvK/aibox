use crate::{components::ChatQuerier, gpt::Openaigpt};

use super::{ChatContext, ChatContextID, ChatMessage, RenderMessageProp, Role};
use dioxus::{
    html::input_data::keyboard_types::{Code, Modifiers},
    prelude::*,
};

pub fn Message(cx: Scope) -> Element {
    let dialogs = use_shared_state::<Vec<ChatContext>>(cx).unwrap();
    let ids = use_shared_state::<ChatContextID>(cx).unwrap();
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
                            id: message.message_id,
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
    let dialogs = use_shared_state::<Vec<ChatContext>>(cx).unwrap();
    let ids = use_shared_state::<ChatContextID>(cx).unwrap();
    let gpt = use_shared_state::<Openaigpt>(cx).unwrap();
    let current_id = ids.read().current_id;
    if current_id == 0 {
        return None;
    }

    let user_input_contents: &UseState<String> = use_state(cx, || "".into());

    println!("{:?}", user_input_contents);

    let send_message_future = use_future(cx, (dialogs, gpt), |(dialogs, gpt)| async move {
        let current_dialog = dialogs
            .read()
            .iter()
            .find(|dialog| dialog.id == current_id)
            .cloned();
        let gpt = gpt.read();

        if let Some(dialog) = current_dialog {
            if dialog.Message.is_empty() {
                return;
            }
            if dialog.Message.last().unwrap().user != Role::User {
                return;
            }

            let response_message = gpt.query(&dialog).await; // todo add type choice
            println!("response_message: {:#?}", response_message);
            let mut binding = dialogs.write();
            let current_dialog = binding
                .iter_mut()
                .find(|dialog| dialog.id == current_id)
                .unwrap();
            current_dialog.Message.push(response_message);
        }

        // let question = dialogs
        //     .read()
        //     .iter()
        //     .find(|dialog| dialog.id == current_id)
        //     .and_then(|dialog| dialog.Message.last())
        //     .and_then(|message| Some(message.content.clone()));
        // if let Some(question) = question {
        //     println!("question: {question:#?},");
        //     let response = reqwest::get("https://dog.ceo/api/breeds/list/all")
        //         .await
        //         .unwrap()
        //         .text()
        //         .await
        //         .unwrap();

        //     println!("{response:#?}, ");
        //     let mut binding = dialogs.write();
        //     let current_dialog = binding
        //         .iter_mut()
        //         .find(|dialog| dialog.id == current_id)
        //         .unwrap();
        //     current_dialog.Message.push(ChatMessage {
        //         message_id: rand::random::<i32>(), // todo
        //         user: Role::Assistant,
        //         content: response,
        //         time: "2020-01-01 00:00:00".to_string(),
        //     });
        // }
    });

    let user_send_message = move || {
        let mut binding = dialogs.write();
        let current_dialog = binding
            .iter_mut()
            .find(|dialog| dialog.id == current_id)
            .unwrap();
        let content = user_input_contents.get().clone();
        current_dialog.Message.push(ChatMessage {
            message_id: rand::random::<i32>(), // todo
            user: Role::User,
            content: content.clone(),
            time: "2020-01-01 00:00:00".to_string(),
        });
        user_input_contents.set(String::new());

        send_message_future.restart();
    };

    cx.render(rsx! {
            div {
                class: "input_container",
                div {
                    "{user_input_contents}"
                }
                textarea {
                    value: "{user_input_contents}",
                    placeholder: "Prompt ...",
                    oninput: move |evt| { user_input_contents.set(evt.value.clone())},
                    onkeyup: move |evt| {
                        println!("{:?}",evt);
                        if evt.code() == Code::Enter && evt.modifiers() == Modifiers::SHIFT && user_input_contents.is_empty() == false {
                            user_send_message();
                        }
                    },
                    // onfocus: move |_| if user_input_contents.as_str().eq("Input ...") { user_input_contents.set(String::new()) },
                    // onblur: move |_| if user_input_contents.is_empty() { user_input_contents.set(default_input_hints.clone()) }

                }
                button {
                    onclick: move |_| user_send_message(),
                    "Send\n「Shift + Enter」"
                }


            }
        })
}
