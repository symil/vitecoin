use super::block_header::BlockHeader;

pub struct BlockWrapper {
    pub header: BlockHeader,
    pub next_blocks_hashes: Vec<u32>,
}

impl BlockWrapper {
    pub fn from_header(header: BlockHeader) -> Self {
        Self {
            header,
            next_blocks_hashes: vec![],
        }
    }
}
