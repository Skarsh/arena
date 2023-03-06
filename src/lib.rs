type MemoryIndex = usize;

#[derive(Debug)]
struct MemoryArena<'a> {
    size: MemoryIndex,
    base: &'a mut [u8],
    used: MemoryIndex,
    temp_count: i32,
}

#[derive(Debug)]
struct TemporaryMemory<'a> {
    arena: &'a MemoryArena<'a>,
    used: MemoryIndex,
}

impl<'a> MemoryArena<'a> {
    pub fn new(size: MemoryIndex, base: &'a mut [u8]) -> Self {
        Self {
            size,
            base,
            used: 0,
            temp_count: 0,
        }
    }

    pub fn alloc<T>(&mut self, _value: T) -> Option<&mut T> {
        let size = std::mem::size_of::<T>();
        assert!(self.used + size <= self.size);

        if self.used + size <= self.size {
            unsafe {
                let slice =
                    std::slice::from_raw_parts_mut(self.base.as_mut_ptr().add(self.used), size);
                let (_header, body, _tail) = slice.align_to_mut::<T>();

                self.used += size;
                Some(body.first_mut().unwrap())
            }
        } else {
            eprintln!("Trying to allocate more memory than we have available");
            None
        }
    }
}

#[derive(Debug, Default)]
struct Entity {
    health: u32,
    stamina: u32,
    mana: u32,
    speed: u32,
    age: u32,
    weight: u32,
    height: u32,
    width: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::alloc::{alloc, Layout};
    use std::slice;

    #[test]
    fn it_works() {
        let capacity: MemoryIndex = 2 * std::mem::size_of::<Entity>();

        unsafe {
            let layout = Layout::new::<[u8; 2 * std::mem::size_of::<Entity>()]>();
            let ptr = alloc(layout);
            let base = slice::from_raw_parts_mut(ptr, capacity);
            for i in 0..capacity {
                base[i] = 0;
            }

            let mut arena = MemoryArena::new(capacity, base);
            let mut entity = arena
                .alloc(Entity {
                    ..Default::default()
                })
                .unwrap();

            entity.health = 0;
            entity.stamina = 1;
            entity.mana = 2;
            entity.speed = 3;
            entity.age = 4;
            entity.weight = 5;
            entity.height = 6;
            entity.width = 7;

            let mut entity2 = arena
                .alloc(Entity {
                    ..Default::default()
                })
                .unwrap();

            entity2.health = 8;
            entity2.stamina = 9;
            entity2.mana = 10;
            entity2.speed = 11;
            entity2.age = 12;
            entity2.weight = 13;
            entity2.height = 14;
            entity2.width = 15;

            println!("arena: {:?}", arena);
        }
    }
}
