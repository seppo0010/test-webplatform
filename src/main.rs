extern crate webplatform;

pub struct Document<'a> {
    inner: webplatform::Document<'a>,
    page: Box<Page>,
}

impl<'a> Document<'a> {
    fn render_page(&mut self) {
        let body = self.inner.element_query("body").unwrap();
        body.html_set(&*self.page.render());
    }

    pub fn init(page: Box<Page>) -> Self {
        let mut d = Document {
            inner: webplatform::init(),
            page: page,
        };
        d.render_page();
        webplatform::spin();
        d
    }

    pub fn set_page(&mut self, page: Box<Page>) {
        self.page = page;
        self.render_page();
    }
}

pub trait Page {
    fn render(&self) -> String;
}

struct MyPage {
    selected: String,
}

impl MyPage {
    fn new() -> Self {
        MyPage { selected: "hello".to_string() }
    }
}

macro_rules! html {
    ($tag:ident $({$event:ident => |mut self| $callback: tt })* [ $($inner:tt)* ] ) => {{
        format!("<{tag}>{inner}</{tag}>",
            tag=stringify!($tag),
            inner=html!($($inner)*),
        )
    }};
    ($str: expr) => {{
        $str
    }};

    ($el:tt $($rest:tt)*) => {{
        format("{}{}", $el, html!($($rest)*))
    }};
}

impl Page for MyPage {
    fn render(&self) -> String {
        html!(
            h1[
                a{click => |mut self| {
                        self.set_selected("Goodbye".to_owned())
                }}[
                    format!("{} world", self.selected)
                ]
            ]
        )
    }
}


fn main() {
    let p = Box::new(MyPage::new());
    Document::init(p);
}
