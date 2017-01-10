#[macro_use(js)]
extern crate webplatform;
extern crate libc;
use std::ffi::CString;
use webplatform::Interop;

/*
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
*/

pub struct Document<'a> {
    inner: webplatform::Document<'a>,
}

impl<'a> Document<'a> {
    pub fn init<C: Component>(component: C) -> Self {
        let d = Document {
            inner: webplatform::init(),
        };
        let j = component.render().js_create(&d.inner);
        js! { (j) b"document.getElementsByTagName('body')[0].appendChild(WEBPLATFORM.rs_refs[$0])" };
        webplatform::spin();
        d
    }
}

pub enum HtmlNode {
    Tag {
        tag_name: String,
        content: Vec<HtmlNode>,
    },
    Text(String),
}

impl HtmlNode {
    fn js_create<'a>(&self, document: &webplatform::Document<'a>) -> i32 {
        match *self {
            HtmlNode::Tag {ref tag_name, ref content}  => {
                let id = js! { (&**tag_name) b"\
                    var el = document.createElement(UTF8ToString($0));\
                    return WEBPLATFORM.rs_refs.push(el) - 1;\
                \0" };
                for child in content.iter() {
                    js! { (id, child.js_create(document)) b"\
                        WEBPLATFORM.rs_refs[$0].appendChild(WEBPLATFORM.rs_refs[$1]);\
                    \0" };
                }
                id
            },
            HtmlNode::Text(ref s) => {
                js! { (&**s) b"\
                    var el = document.createTextNode(UTF8ToString($0));\
                    return WEBPLATFORM.rs_refs.push(el) - 1;\
                \0" }
            }
        }
    }
}

pub trait Component: Sized {
    fn render(&self) -> HtmlNode;
}

struct MyComponent {
    selected: String,
}

impl MyComponent {
    fn new() -> Self {
        MyComponent { selected: "hello".to_string() }
    }

    fn selected(&self) -> &str {
        &*self.selected
    }

    fn set_selected(&mut self, _event: webplatform::Event) {
        self.selected = "goodbye".to_owned();
    }
}

impl Component for MyComponent {
    fn render(&self) -> HtmlNode {
        HtmlNode::Tag {
            tag_name: "a".to_owned(),
            content: vec![
                HtmlNode::Text(format!("{} world", self.selected)),
            ],
        }
    }
}


fn main() {
    let p = MyComponent::new();
    Document::init(p);
}
