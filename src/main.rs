use digest::generic_array::{typenum, GenericArray};
use digest::{Digest, FixedOutput, Reset, Update};
use md4::Md4;

#[derive(Clone, Debug, Default)]
struct Ed2kHash {
    md4_hasher: Md4,
    current_chunk_len: usize,
    num_full_chunks: usize,
    chunk_hasher: Md4,
}

impl Ed2kHash {
    pub const CHUNK_SIZE: usize = 9_728_000;

    pub fn new() -> Self {
        Default::default()
    }

    fn finalize_chunk(&mut self) {
        self.num_full_chunks += 1;
        Digest::update(
            &mut self.chunk_hasher,
            self.md4_hasher.finalize_reset().as_slice(),
        );
        self.current_chunk_len = 0;
    }

    fn chunk_remaining(&self) -> usize {
        Self::CHUNK_SIZE - self.current_chunk_len
    }
}

impl Update for Ed2kHash {
    fn update(&mut self, input: impl AsRef<[u8]>) {
        let input_ref = input.as_ref();

        if input_ref.len() == 0 {
            return;
        }

        let mut input_start = 0;
        while input_ref[input_start..].len() > 0 {
            let read_len = std::cmp::min(self.chunk_remaining(), input_ref.len() - input_start);

            if self.chunk_remaining() == 0 {
                self.finalize_chunk();
            }

            Digest::update(
                &mut self.md4_hasher,
                &input_ref[input_start..input_start + read_len],
            );
            self.current_chunk_len += read_len;

            input_start += read_len;
        }
    }
}

impl Reset for Ed2kHash {
    fn reset(&mut self) {
        self.md4_hasher = Default::default();
        self.chunk_hasher = Default::default();
        self.num_full_chunks = 0;
        self.current_chunk_len = 0;
    }
}

impl FixedOutput for Ed2kHash {
    type OutputSize = typenum::U16;

    fn finalize_into(mut self, out: &mut GenericArray<u8, Self::OutputSize>) {
        if self.num_full_chunks < 2 {
            self.md4_hasher.finalize_into(out);
        } else {
            self.finalize_chunk();
            self.chunk_hasher.finalize_into(out);
        }
        self.num_full_chunks = 0;
        self.current_chunk_len = 0;
    }

    fn finalize_into_reset(&mut self, out: &mut GenericArray<u8, Self::OutputSize>) {
        if self.num_full_chunks == 0 {
            self.md4_hasher.finalize_into_reset(out);
        } else {
            self.finalize_chunk();
            self.chunk_hasher.finalize_into_reset(out);
        }
        self.num_full_chunks = 0;
        self.current_chunk_len = 0;
    }
}

fn main() {
    println!("Hello, world!");
}
