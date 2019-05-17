use askama::Template;

pub struct Foo {
    bar: i64,
}

pub struct ParentTemplate {
    pub foo: Foo,
}

#[derive(Template)]
#[template(path = "child.html")]
pub struct ChildTemplate {
    _parent: ParentTemplate,
}

#[derive(Template)]
#[template(path = "intermediate.html")]
pub struct IntermediateTemplate {
    _parent: ParentTemplate,
}

#[derive(Template)]
#[template(path = "subchild.html")]
pub struct SubChildTemplate {
    _parent: IntermediateTemplate,
}

fn main() {}
