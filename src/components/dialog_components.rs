use super::{ChatContext, ChatContextID};
use dioxus::prelude::*;

pub fn Dialog(cx: Scope) -> Element {
    cx.render(rsx! {
        Dialog_tab(cx),
        Dialog_control(cx),
    })
}

pub fn Dialog_tab(cx: Scope) -> Element {
    let dialogs = use_shared_state::<Vec<ChatContext>>(cx).unwrap();
    let ids = use_shared_state::<ChatContextID>(cx).unwrap();
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
    let dialogs = use_shared_state::<Vec<ChatContext>>(cx).unwrap();
    let ids = use_shared_state::<ChatContextID>(cx).unwrap();
    cx.render(rsx! {
        div {
            class: "dialog_control",
            // buttom thats add new dialog
            button {
                class: "control_button",
                onclick: move |_| {
                    let new_id = ids.read().next_id;
                    dialogs.write().push(ChatContext {
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
