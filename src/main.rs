#[macro_use(js)]
extern crate webplatform;
extern crate libc;
use std::ffi::CString;
use std::rc::Rc;
use std::cell::RefCell;
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
        events: Vec<(String, Rc<Box<FnMut(webplatform::Event)>>)>,
    },
    Text(String),
}

impl HtmlNode {
    fn js_create<'a>(&self, doc: &webplatform::Document<'a>) -> i32 {
        match *self {
            HtmlNode::Tag {ref tag_name, ref content, ref events}  => {
                let id = js! { (&**tag_name) b"\
                    var el = document.createElement(UTF8ToString($0));\
                    return WEBPLATFORM.rs_refs.push(el) - 1;\
                \0" };
                for child in content.iter() {
                    js! { (id, child.js_create(doc)) b"\
                        WEBPLATFORM.rs_refs[$0].appendChild(WEBPLATFORM.rs_refs[$1]);\
                    \0" };
                }
                for event in events.iter() {
                    let mut f = event.1.clone();
                    webplatform::HtmlNode { id: id, doc: doc }.on(&*event.0, move |e| {
                            match Rc::get_mut(&mut f) {
                                Some(ref mut s) => s(e),
                                None => panic!("cannot unwrap"),
                            }
                    });
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
    fn render(self) -> HtmlNode;
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
    fn render(self) -> HtmlNode {
        let s = Rc::new(RefCell::new(self));
        HtmlNode::Tag {
            tag_name: "a".to_owned(),
            content: vec![
                HtmlNode::Text(s.clone().borrow().selected().to_owned()),
            ],
            events: vec![
                ("click".to_owned(), {
                    let me = s.clone();
                    Rc::new(Box::new(move |_e| {
                        webplatform::alert(me.borrow().selected());
                    }))
                }),
                ("mouseover".to_owned(), {
                    let me = s.clone();
                    Rc::new(Box::new(move |_e| {
                        webplatform::alert(&*format!("AAA: {}", me.borrow().selected()));
                    }))
                }),
            ],
        }
    }
}


fn main() {
    let p = MyComponent::new();
    Document::init(p);
}
