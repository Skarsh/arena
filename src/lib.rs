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
        const SIZE: MemoryIndex = 100_000_000;

        unsafe {
            let layout = Layout::new::<[u8; SIZE]>();
            let ptr = alloc(layout);
            let base = slice::from_raw_parts_mut(ptr, SIZE);
            for i in 0..SIZE {
                base[i] = 0;
            }

            let mut arena = MemoryArena::new(SIZE, base);
            let mut entity = arena
                .alloc(Entity {
                    ..Default::default()
                })
                .unwrap();
            entity.age = 29;

            let mut entity2 = arena
                .alloc(Entity {
                    ..Default::default()
                })
                .unwrap();
            entity2.age = 42;
        }
    }
}
