use libmdbx::{Environment, Mode, Flags};
use std::path::Path;

fn main() {
    let path = Path::new("./crawler_db");
    let env = Environment::new()
        .set_max_dbs(10)
        .open_with_permissions(path, Mode::ReadWrite, 0o644)
        .expect("No se pudo abrir la DB. ¿Está el relayer corriendo?");

    // Abrimos la tabla de peers (PEER_INFO_TABLE suele llamarse "peer-info")
    let tx = env.begin_ro_txn().unwrap();
    let db = tx.open_db(Some("peer-info")).expect("No se encontró la tabla peer-info");
    let mut cursor = tx.cursor_open_db(db).unwrap();

    println!("=== CONTENIDO BINARIO DE PEER_INFO_TABLE ===");
    for result in cursor.iter() {
        let (key, value) = result.unwrap();
        // Intentamos imprimir el ID del peer (suelen ser strings o bytes legibles)
        println!("Peer ID (hex): {:x?}", key);
        println!("Datos (longitud): {} bytes", value.len());
    }
}