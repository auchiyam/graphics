/// Simple representation of the buffers being passed around
pub struct Buffer {
    /// Storage for actual data in binary format
    pub buffer: Vec<u8>,
    /// amount of offsets that must be traversed to access the nth value in
    /// buffer
    pub offsets: Vec<usize>,
}

pub trait ToBuffer {
    fn to_buffer(&self) -> Buffer;
}

impl Buffer {
    pub fn get(&self, index: usize) -> Option<&[u8]> {
        // if index points above the amount of partition, it's faulty so return empty
        if index > self.offsets.len() {
            return None;
        }

        match (self.offsets.get(index - 1), self.offsets.get(index)) {
            /* empty, meaning no partitions and buffer should all be returned */
            (None, None) => &self.buffer,
            /* index is 0 because subtraction went out of  bounds */
            (None, Some(&r)) => &self.buffer[0..r],
            // index is the last item, so it should take all
            (Some(&l), None) => &self.buffer[l..],
            // middle value
            (Some(&l), Some(&r)) => &self.buffer[l..r],
        }
        .into()
    }
}
