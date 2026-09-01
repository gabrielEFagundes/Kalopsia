use std::{any::TypeId, collections::HashMap, sync::{LazyLock, Mutex}};

static POOL: LazyLock<Mutex<HashMap<TypeId, IdPool>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier(pub i32);

pub struct IdPool{
    next: i32,
    free: Vec<i32>
}

#[allow(dead_code)]
impl IdPool{
    fn new() -> Self{
        Self { next: 1, free: Vec::new() }
    }

    fn next(&mut self) -> i32{
        match self.free.pop(){
            Some(id) => id,
            None => {
                let next = self.next;
                self.next+=1;
                next
            }
        }
    }

    fn free(&mut self, id: i32){
        self.free.push(id);
    }

    fn with_pool<T: 'static, R>(f: impl FnOnce(&mut IdPool) -> R) -> R{
        let mut map = POOL.lock().unwrap();
        let pool = map.entry(TypeId::of::<T>()).or_insert_with(|| IdPool::new());
        f(pool)
    }
}

pub trait HasId{
    fn alloc_id(&mut self);
    fn free_id(&mut self);
}