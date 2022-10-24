use crate::browser;
use futures::channel::mpsc::{unbounded, UnboundedReceiver};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub fn add_click_handler(elem: HtmlElement) -> UnboundedReceiver<()> {
    let (mut click_sender, click_receiver) = unbounded();
    let on_click = browser::closure_wrap(Box::new(move || {
        if let Err(err) = click_sender.start_send(()) {
            error!("Error starting sending {:#?}", err);
        };
    }) as Box<dyn FnMut()>);
    elem.set_onclick(Some(on_click.as_ref().unchecked_ref()));
    on_click.forget();
    click_receiver
}
