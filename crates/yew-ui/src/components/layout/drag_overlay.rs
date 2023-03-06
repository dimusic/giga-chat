use stylist::yew::styled_component;
use yew::{classes, html, Callback, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct DragOverlayProps {
    pub on_done: Callback<()>,
}

#[styled_component(DragOverlay)]
pub fn drag_overlay(props: &DragOverlayProps) -> Html {
    let container_styles = css! {
        r#"
            display: flex;
            align-items: center;
            justify-content: center;
            position: fixed;
            top: 2px;
            bottom: 2px;
            left: 2px;
            right: 2px;
            background: rgba(0, 0, 0, 0.5);
            outline: #8b50ff solid 2px;
            z-index: 999;
        "#
    };

    let handle_done_click = {
        let on_done = props.on_done.clone();

        Callback::from(move |_| on_done.emit(()))
    };

    html! {
        <div data-tauri-drag-region="true" class={container_styles}>
            <button class={classes!("btn", "btn-success")} onclick={handle_done_click}>{ "Lock" }</button>
        </div>
    }
}
