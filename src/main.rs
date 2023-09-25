mod template;

use template::build_template;

use std::path::Path;

fn main() {
    build_template(&Path::new("templates"), &Path::new("test"));
}
