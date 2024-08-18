mod comment;
mod container;
mod etc_host_writer;
mod h;
mod host_collections;
mod host_reader;
mod list;
mod list_writer;
mod r;
mod storage_path;

pub use comment::is_comment;
pub use container::Container;
pub use etc_host_writer::etc_write;
pub use h::H;
pub use host_reader::{etc_host_reader, host_reader};
pub use list::HashList;
pub use list::VecList;
pub use list_writer::write_list;
pub use r::R;
pub use storage_path::StoragePath;

pub enum Cap {
    Capacity(usize),
    None,
}
