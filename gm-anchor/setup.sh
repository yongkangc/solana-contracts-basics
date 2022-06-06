solana-keygen new -o id.json
solana airdrop 2 $(solana-keygen pubkey ./id.json)
export ANCHOR_WALLET='../id.json'

