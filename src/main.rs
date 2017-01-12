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
        let mut c = OuterComponent::new(component);
        let j = c.node.js_create(&d.inner);
        js! { (j) b"document.getElementsByTagName('body')[0].appendChild(WEBPLATFORM.rs_refs[$0])" };
        webplatform::spin();
        d
    }
}

pub struct HtmlNode {
    id: i32,
    content: HtmlNodeContent,
}

impl HtmlNode {
    fn js_create<'a>(&mut self, doc: &webplatform::Document<'a>) -> i32 {
        self.id = self.content.js_create(doc);
        self.id
    }

    fn js_update<'a>(&mut self, doc: &webplatform::Document<'a>) {
        self.content.js_update(self.id, doc);
    }
}

pub enum HtmlNodeContent {
    Tag {
        tag_name: String,
        children: Vec<HtmlNode>,
        events: Vec<(String, Rc<Box<FnMut(webplatform::Event)>>)>,
    },
    Text(String),
}

impl HtmlNodeContent {
    fn js_create<'a>(&mut self, doc: &webplatform::Document<'a>) -> i32 {
        match *self {
            HtmlNodeContent::Tag {ref tag_name, ref mut children, ref events}  => {
                let id = js! { (&**tag_name) b"\
                    var el = document.createElement(UTF8ToString($0));\
                    return WEBPLATFORM.rs_refs.push(el) - 1;\
                \0" };
                for child in children.iter_mut() {
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
            HtmlNodeContent::Text(ref s) => {
                js! { (&**s) b"\
                    var el = document.createTextNode(UTF8ToString($0));\
                    return WEBPLATFORM.rs_refs.push(el) - 1;\
                \0" }
            }
        }
    }

    fn js_update<'a>(&mut self, id: i32, _doc: &webplatform::Document<'a>) {
        match *self {
            HtmlNodeContent::Tag {tag_name: _, children: _, events: _}  => {
            },
            HtmlNodeContent::Text(ref s) => {
                js! { (id, &**s) b"\
                    WEBPLATFORM.rs_refs[$0].textContent = UTF8ToString($1)\
                \0" };
            }
        }
    }
}

struct OuterComponent<C: Component> {
    inner: Rc<RefCell<C>>,
    node: HtmlNode,
}

impl<C: Component> OuterComponent<C> {
    fn new(inner: C) -> Self {
        let inner = Rc::new(RefCell::new(inner));
        OuterComponent {
            inner: inner.clone(),
            node: C::render(inner.clone()),
        }
    }
}

pub trait Component: Sized {
    fn render(s: Rc<RefCell<Self>>) -> HtmlNode;
}

struct MyComponent {
    selected: String,
    count: u32,
}

impl MyComponent {
    fn new() -> Self {
        MyComponent { selected: "hello".to_string(), count: 0 }
    }

    fn selected(&self) -> &str {
        &*self.selected
    }

    fn set_selected(&mut self, _event: webplatform::Event) {
        self.selected = "goodbye".to_owned();
    }

    fn counter(&self) -> u32 {
        self.count
    }

    fn inc(&mut self) {
        self.count += 1;
    }
}

impl Component for MyComponent {
    fn render(s: Rc<RefCell<Self>>) -> HtmlNode {
        HtmlNode {
            id: -1,
            content: HtmlNodeContent::Tag {
                tag_name: "a".to_owned(),
                children: vec![{
                    let a = s.clone();
                    let b = a.borrow();
                    HtmlNode { id: -1, content: HtmlNodeContent::Text(b.selected().to_owned()) }
                }],
                events: vec![
                    ("click".to_owned(), {
                        let me = s.clone();
                        Rc::new(Box::new(move |_e| {
                            let s = me.borrow();
                            webplatform::alert(&*format!("{}{}", s.counter(), s.selected()));
                        }))
                    }),
                    ("mouseover".to_owned(), {
                        let me = s.clone();
                        Rc::new(Box::new(move |_e| {
                            let mut me = me.borrow_mut();
                            me.inc();
                        }))
                    }),
                ],
            }
        }
    }
}


fn main() {
    let p = MyComponent::new();
    Document::init(p);
}
