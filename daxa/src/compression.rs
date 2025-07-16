// compression.rs 
use crate::{Result, DaxaError};

#[deriveDebug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CompressionAlgorithm {
    None,
    Lz4,
    Zstd,
}

pub fn compress(data: &[u8], algorithm: CompressionAlgorithm, level: Option<i32>) -> Result<Vec<u8>> {
    match algorithm {
        CompressionAlgorithm::None => Ok(data.to_vec()),
        CompressionAlgorithm::Lz4 => {
            // lz4_flex default compression level is usually good.
            // Level for lz4_flex isn't directly exposed like zstd, it's more about fast modes.
            // We'll ignore `level` for lz4 for simplicity here.
            lz4_flex::compress_prepend_size(data)
                .map_err(|e| DaxaError::Compression(format!("LZ4 compression error: {}", e)))
        }
        CompressionAlgorithm::Zstd => {
            let comp_level = level.unwrap_or(zstd::DEFAULT_COMPRESSION_LEVEL); // zstd default is 3
            zstd::encode_all(data, comp_level)
                .map_err(|e| DaxaError::Compression(format!("Zstd compression error: {}", e)))
        }
    }
}

pub fn decompress(compressed_data: &[u8], algorithm: CompressionAlgorithm) -> Result<Vec<u8>> {
    match algorithm {
        CompressionAlgorithm::None => Ok(compressed_data.to_vec()),
        CompressionAlgorithm::Lz4 => {
            lz4_flex::decompress_size_prepended(compressed_data)
                .map_err(|e| DaxaError::Compression(format!("LZ4 decompression error: {}", e)))
        }
        CompressionAlgorithm::Zstd => {
            zstd::decode_all(compressed_data)
                .map_err(|e| DaxaError::Compression(format!("Zstd decompression error: {}", e)))
        }
    }
}

// TODO: Add auto-detect compression based on flags in data stream/block header