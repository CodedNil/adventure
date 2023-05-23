use dioxus::prelude::*;

pub fn create() {
    dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
    let journal_width = 400.0;

    cx.render(rsx! {
        div {
            display: "flex",
            height: "calc(100vh - 16px)",
            div {
                class: "main_content",
                flex: "1",
                padding: "0",
                background_color: "#C5E1A5",
                border_radius: "10px",
                "Main Content"
            }
            div {
                class: "separator",
                resize: "horizontal",
                cursor: "col-resize",
                width: "10px",
            }
            div {
                class: "side_content",
                width: journal_width,
                background_color: "#FFF59D",
                border_radius: "10px",
                "Side Content"
            }
        }
    })
}
