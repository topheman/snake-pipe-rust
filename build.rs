use ::static_files::resource_dir;

pub fn main() -> std::io::Result<()> {
    resource_dir("./static").build()
}
