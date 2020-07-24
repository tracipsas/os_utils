pub const HASH_BLOCK_SIZE: usize = 1024;

pub trait DigestReader: digest::Digest {
    fn digest_reader(
        &mut self,
        source: &mut dyn std::io::Read,
    ) -> std::io::Result<digest::generic_array::GenericArray<u8, Self::OutputSize>> {
        let mut buffer = [0u8; HASH_BLOCK_SIZE];
        loop {
            let nb_bytes_read = source.read(&mut buffer)?;
            self.update(&buffer[..nb_bytes_read]);
            if nb_bytes_read == 0 {
                break;
            }
        }
        Ok(self.finalize_reset())
    }
}
impl<D: digest::Digest> DigestReader for D {}
