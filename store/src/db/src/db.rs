use std::sync::{Arc, Mutex};
use gluesql::memory_storage::MemoryStorage;
use gluesql::prelude::{Glue, parse};

pub fn create_db() -> Arc<Mutex<Glue<MemoryStorage>>> {
    let storage = MemoryStorage::default();
    let mut glue = Glue::new(storage);

    let sqls = "
        CREATE TABLE Glue(id INTEGER);
        INSERT INTO Glue VALUES (100);
        INSERT INTO Glue VALUES (200);
        SELECT * FROM Glue Where id > 100;
        DROP TABLE Glue;
    ";

    for query in parse(sqls).unwrap() {
        glue.execute(&query).unwrap();
    }

    return Arc::new(Mutex::new(glue));
}
