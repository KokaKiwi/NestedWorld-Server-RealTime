use db::Database;
use mioco::sync::Mutex;
use slab::Slab;
use std::sync::Arc;
use super::Combat;

type Shared<T> = Arc<Mutex<T>>;
fn share<T>(value: T) -> Shared<T> {
    Arc::new(Mutex::new(value))
}

pub struct CombatStore {
    db: Database,
    entries: Slab<Shared<Combat>, usize>
}

impl CombatStore {
    pub fn new(db: Database) -> Arc<Mutex<CombatStore>> {
        share(CombatStore {
            db: db,
            entries: Slab::new(32),
        })
    }

    pub fn create(&mut self) -> Arc<Mutex<Combat>>
    {
        if !self.entries.has_remaining() {
            self.entries.grow(16);
        }

        // Should not be None, as we ensured that the slab was sufficiently
        // big.
        let entry = self.entries.vacant_entry().unwrap();

        let combat = share(Combat::new(self.db.clone(), entry.index() as u32));
        entry.insert(combat.clone());

        combat
    }

    pub fn get(&self, id: u32) -> Option<Arc<Mutex<Combat>>> {
        self.entries.get(id as usize).map(Clone::clone)
    }

    pub fn remove(&mut self, id: u32) -> bool {
        self.entries.remove(id as usize).is_some()
    }
}
