#[macro_use(js)]
extern crate webplatform;

use std::rc::Rc;

trait Renderable {
    fn html(&self) -> String;
}

#[derive(Default)]
struct SubmitForm {}

impl SubmitForm {}

impl Renderable for SubmitForm {
    fn html(&self) -> String {
        "<form><input type='text' name='message'></form>".to_owned()
    }
}

fn main() {
    let document = Rc::new(webplatform::init());
    let body = document.element_query("body").unwrap();
    let submit_form = SubmitForm::default();
    body.html_set(&*submit_form.html());
    let form = document.element_query("form").unwrap();
    form.on("submit", move |e| {
        let document = document.clone();
        e.prevent_default();
        webplatform::ajax_get(&*document.clone(), "data", move |s| {
            let body = document.element_query("body").unwrap();
            body.html_set(&*format!("<pre>{:?}\n{}</pre>",
                s.as_result(),
                s.response_text().unwrap()));
        });
    });
}
