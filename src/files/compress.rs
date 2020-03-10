use ::std::io::Read;

use ::brotli::enc::BrotliEncoderParams;
use ::lazy_static::lazy_static;

use crate::header::CompressionAlg;
use crate::util::FedResult;

lazy_static! {
    static ref BROTLI_CONFIG: BrotliEncoderParams = {
        let mut param = BrotliEncoderParams::default();
        param.quality = 6;
        param
    };
}

pub fn compress_file(data: Vec<u8>, alg: &CompressionAlg) -> FedResult<Vec<u8>> {
    match alg {
        CompressionAlg::Brotli => brotli_compress(&data),
        CompressionAlg::None => Ok(data),
    }
}

pub fn brotli_compress(data: &[u8]) -> FedResult<Vec<u8>> {
    let mut compress = brotli::CompressorReader::new(data, 4096, 6, 22);
    let mut output = Vec::with_capacity(data.len());
    match compress.read_to_end(&mut output) {
        Ok(len) => {
            if len > 0 {
                Ok(output)
            } else {
                Err("No data was read during compression".to_owned())
            }
        }
        Err(err) => Err(format!("Brotli compress error: {}", err)),
    }
}

pub fn decompress_file(data: Vec<u8>, alg: &CompressionAlg) -> FedResult<Vec<u8>> {
    match alg {
        CompressionAlg::Brotli => brotli_decompress(&data),
        CompressionAlg::None => Ok(data),
    }
}

pub fn brotli_decompress(data: &[u8]) -> FedResult<Vec<u8>> {
    let mut decompress = brotli::Decompressor::new(data, 4096);
    let mut output = Vec::with_capacity(data.len());
    match decompress.read_to_end(&mut output) {
        Ok(_) => Ok(output),
        Err(err) => Err(format!("Brotli decompress error: {}", err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brotli_compression_empty() {
        let input = vec![];
        let actual = brotli_compress(&input).unwrap();
        let expected = vec![59u8];
        assert_eq!(expected, actual);
    }

    #[test]
    fn brotli_compression() {
        let input = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
        ];
        let expected = vec![
            27, 63, 0, 0, 196, 3, 224, 120, 26, 226, 75, 49, 9, 126, 86, 64, 57, 221, 231, 199, 0,
            16, 86, 3,
        ];
        let actual = brotli_compress(&input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn brotli_decompression_empty() {
        let input = vec![59];
        let actual = brotli_decompress(&input).unwrap();
        let expected: Vec<u8> = vec![];
        assert_eq!(expected, actual);
    }

    #[test]
    fn brotli_decompression() {
        let expected = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
            16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
        ];
        let input = vec![
            27, 63, 0, 0, 196, 3, 224, 120, 26, 226, 75, 49, 9, 126, 86, 64, 57, 221, 231, 199, 0,
            16, 86, 3,
        ];
        let actual = brotli_decompress(&input).unwrap();
        assert_eq!(expected, actual);
    }
}
