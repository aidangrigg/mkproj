mod template;

use template::Template;

use std::path::Path;

fn main() {
    let _t = Template::new(Path::new("templates"), Path::new("test"));
}
