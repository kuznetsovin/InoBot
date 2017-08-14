use rusqlite::Connection;
use config::Config;

pub struct Store {
    conn: Connection
}

impl Store {
    pub fn connect(conf: &Config) -> Store {
        let conn = Connection::open(&conf.db_path).unwrap();
        conn.execute("CREATE TABLE IF NOT EXISTS chats (id INTEGER);", &[]).unwrap();
        Store {
            conn: conn,
        }
    }

    pub fn add_chart(&self, chat_id: i64) {
        &self.conn.execute(
            "INSERT INTO chats (id) VALUES (?1)",
            &[&chat_id]
        ).unwrap();
    }

    pub fn get_chart_ids(&self) -> Vec<i64> {
        let mut stmt = self.conn.prepare("SELECT id FROM chats").unwrap();
        let chat_iter = stmt.query_map(&[], |row| {row.get(0)}).unwrap();

        let mut charts: Vec<i64> = Vec::new();
        for c in chat_iter {
          charts.push(c.unwrap());
        };

        charts
    }
}