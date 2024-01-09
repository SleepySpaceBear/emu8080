pub trait MemoryAccess {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, val: u8);
}

pub struct Memory<const N: usize> {
    arr: [u8; N]
}

impl<const N: usize> Memory<N> {
    pub fn new() -> Self {
        return Memory {
            arr: [0 as u8; N]
        }
    }
}

impl<const N: usize> MemoryAccess for Memory<N> {
    fn read(&self, addr: u16) -> u8 {
        if (addr as usize) >= N {
            return 0;
        }

        return self.arr[addr as usize]
    }

    fn write(&mut self, addr: u16, val: u8) {
        if (addr as usize) < N {
            self.arr[addr as usize] = val
        }
    }
}

