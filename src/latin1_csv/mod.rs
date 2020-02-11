pub trait FromLatin1: Sized {
    fn from_latin1(path: &std::path::Path) -> std::io::Result<Self>;
}

impl FromLatin1 for csv::Reader<encoding_rs_io::DecodeReaderBytes<std::fs::File, std::vec::Vec<u8>>> {
    fn from_latin1(path: &std::path::Path) -> std::io::Result<Self> {
        let input = encoding_rs_io::DecodeReaderBytesBuilder::new()
            .encoding(Some(encoding_rs::WINDOWS_1252))
            .build(std::fs::File::open(path)?);
        Ok(csv::ReaderBuilder::new().from_reader(input))
    }
}
