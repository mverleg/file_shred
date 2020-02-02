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


fn brotli_compress(data: &[u8]) -> FedResult<Vec<u8>> {
    let mut compress = brotli::CompressorReader::new(data, 4096, 6, 22);
    let mut output = Vec::with_capacity(data.len());
    match compress.read_to_end(&mut output) {
        Ok(len) => match data.len() == len {
            true => Ok(output),
            false => Err("Not all data was read during compression".to_owned())
        },
        Err(err) => Err(format!("Brotli error: {}", err)),
    }
}


//TODO @mark: test
