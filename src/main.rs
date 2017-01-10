#[macro_use(js)]
extern crate webplatform;
extern crate libc;
use std::ffi::CString;
use webplatform::Interop;

use std::rc::Rc;
use std::cell::{RefCell,RefMut};

macro_rules! html {
    ($document: ident, $me: ident, $tag:ident $({ $event:ident => $callback: block })* [ $($inner:tt)* ] ) => {{
        let id = js! { (stringify!($tag), html!($document, $me, $($inner)*)) b"\
            var el = document.createElement(UTF8ToString($0));\
            el.appendChild(WEBPLATFORM.rs_refs[$1]);\
            return WEBPLATFORM.rs_refs.push(el) - 1;\
        \0" };
        $(
            let me = $me.clone();
            webplatform::HtmlNode { id: id, doc: $document }.on(stringify!($event), move |e| {
                let f = $callback;
                f(e, me.borrow_mut());
            });
        )*
        id
    }};
    ($document: ident, $me: ident, $str: expr) => {{
        js! { (&*$str) b"\
            var el = document.createTextNode(UTF8ToString($0));\
            return WEBPLATFORM.rs_refs.push(el) - 1;\
        \0" }
    }};
}

pub struct Document<'a> {
    inner: webplatform::Document<'a>,
}

impl<'a> Document<'a> {
    pub fn init<P: Page>(page: P) -> Self {
        let d = Document {
            inner: webplatform::init(),
        };
        let j = page.render(&d.inner);
        js! { (j) b"document.getElementsByTagName('body')[0].appendChild(WEBPLATFORM.rs_refs[$0])" };
        webplatform::spin();
        d
    }
}

pub trait Page {
    fn render<'a>(self, document: &webplatform::Document<'a>) -> i32;
}

struct MyPage {
    selected: String,
}

impl MyPage {
    fn new() -> Self {
        MyPage { selected: "hello".to_string() }
    }

    fn selected(&self) -> &str {
        &*self.selected
    }

    fn set_selected(&mut self, _event: webplatform::Event) {
        self.selected = "goodbye".to_owned();
    }
}

impl Page for MyPage {
    fn render<'a>(self, document: &webplatform::Document<'a>) -> i32 {
        let me = Rc::new(RefCell::new(self));
        html!(document, me,
            h1[
                a{click => { |event, mut me: RefMut<MyPage>| {
                    me.set_selected(event);
                }}}[
                    format!("{} world", me.borrow().selected())
                ]
            ]
        )
    }
}


fn main() {
    let p = MyPage::new();
    Document::init(p);
}
