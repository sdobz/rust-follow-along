struct Context<'s>(&'s str);

struct Parser<'c, 's: 'c> {
    // s will live at least as long as c
    context: &'c Context<'s>,
}

impl<'c, 's> Parser<'c, 's> {
    fn parse(&self) -> Result<(), &'s str> {
        Err(&self.context.0[1..])
    }
}

fn parse_context(context: Context) -> Result<(), &str> {
    Parser { context: &context }.parse()
}

trait Red {}

struct Ball<'a> {
    diameter: &'a i32,
}

impl<'a> Red for Ball<'a> {}

pub fn demo() {
    let num = 5;

    let _obj = Box::new(Ball { diameter: &num }) as Box<dyn Red>;
}

struct StrWrap<'a>(&'a str);

fn foo<'a>(string: &'a str) -> StrWrap<'a> {
    StrWrap(string)
}

fn foo_anon_lifetime(string: &str) -> StrWrap<'_> {
    StrWrap(string)
}

//impl<'a> fmt::Debug for StrWrap<'a> {
//impl fmt::Debug for StrWrap<'_> {
