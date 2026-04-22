pub struct Program {
    pub name: &'static str,
    pub entry: fn(),
}

static PROGRAMS: &[Program] = &[
    Program {
        name: "hello",
        entry: crate::programs::hello::run,
    },
];

pub fn find(name: &str) -> Option<fn()> {
    for p in PROGRAMS {
        if p.name == name {
            return Some(p.entry);
        }
    }
    None
}
