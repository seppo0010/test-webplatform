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
	let body = document.element_query("body").unwrap();
	let submit_form = SubmitForm::default();
	body.html_set(&*submit_form.html());
	let form = document.element_query("form").unwrap();
	form.on("submit", |e| {
e.prevent_default();
		webplatform::alert(&*e.target.unwrap().html_get());
	});
}
