#[macro_use(js)]
extern crate webplatform;

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
    let document = webplatform::init();
    webplatform::ajax_get(&document, "data", move |s| {
        let body = document.element_query("body").unwrap();
        body.html_set(&*format!("<pre>{:?}\n{}</pre>",
            s.as_result(),
            s.response_text().unwrap()));
    });
}
