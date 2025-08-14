pub trait MemoryAccess {
    fn read_byte(&self, addr: u16) -> u8;
    fn write_byte(&mut self, addr: u16, val: u8);

    fn read_bytes(&self, addr: u16, count: u16) -> &[u8];
    fn write_bytes(&mut self, addr: u16, val: &[u8]);
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

    pub fn copy_into_from_slice(&mut self, src: &[u8], index: u16) {
        let end = std::cmp::min(index as usize + src.len(), N);
        self.arr[index as usize..end].copy_from_slice(src);
    }

    pub fn get_bytes(&self, start: u16, end: u16) -> &[u8] {
        &self.arr[start as usize..end as usize]
    }
}

impl<const N: usize> MemoryAccess for Memory<N> {
    fn read_byte(&self, addr: u16) -> u8 {
        if (addr as usize) >= N {
            return 0;
        }

        return self.arr[addr as usize]
    }

    fn write_byte(&mut self, addr: u16, val: u8) {
        if (addr as usize) < N {
            self.arr[addr as usize] = val
        }
    }

    fn read_bytes(&self, addr: u16, count: u16) -> &[u8] {
        let start = addr as usize;
        let end = start + count as usize;
        &self.arr[start..end]
    }

    fn write_bytes(&mut self, addr: u16, val: &[u8]) {
        let start = addr as usize;
        let end = start + val.len();
        self.arr[start..end].copy_from_slice(val);
    }
}

