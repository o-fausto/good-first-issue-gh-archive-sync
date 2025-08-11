use std::io::{Read, BufReader, Cursor};
use flate2::read::GzDecoder;

/// Downloads and decompresses a gzipped file from the given URL.
/// Returns a BufReader over the decompressed data.
pub fn download_and_decompress(url: &str) -> Result<BufReader<GzDecoder<Cursor<Vec<u8>>>>, Box<dyn std::error::Error>> {
    let response = ureq::get(url).call();
    if response.is_err() {
        return Err(format!("Error fetching data from URL: {}", url).into());
    }
    let response = response.unwrap();

    let mut gzipped_data = Vec::new();
    response.into_reader().read_to_end(&mut gzipped_data)?;
    let cursor = Cursor::new(gzipped_data);
    let decoder = GzDecoder::new(cursor);
    Ok(BufReader::new(decoder))
}