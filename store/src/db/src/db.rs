use gluesql::memory_storage::MemoryStorage;
use gluesql::prelude::Glue;

// todo: should probably be mutex; see https://users.rust-lang.org/t/sharing-a-struct-between-other-structs/54591
pub fn db_cfg() -> Glue<MemoryStorage> {
    let storage = MemoryStorage::default();
    let mut glue = Glue::new(storage);

    // todo: do inserts here; see https://crates.io/crates/gluesql/0.2.1

    return glue;
}
