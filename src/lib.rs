mod abi;
mod pb;
use hex_literal::hex;
use pb::erc721;
use substreams_ethereum::{pb::eth::v2 as eth};

/// Our NFT contract of interest
const TRACKED_CONTRACT: [u8;20] = hex!("CF3bc939f9B2487092936F21cC0757b2b523B7aA");

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_transfers(blk: eth::Block) -> Result<erc721::Transfers, substreams::errors::Error> {
    Ok(erc721:: Transfers {
            transfers: blk
                .events::<abi::erc721::events::Transfer>(&[&TRACKED_CONTRACT])
                .map(|(transfer, log)| {
                    substreams::log::info!("NFT Transfer");

                    erc721::Transfer {
                        trx_hash: log.receipt.transaction.hash.clone(),
                        from: transfer.from,
                        to: transfer.to,
                        token_id: transfer.token_id.low_u64(),
                        ordinal: log.block_index() as u64,
                    }
                })
                .collect(),
    })
}
