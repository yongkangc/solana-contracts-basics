# Anchor GM Program
## Setup
`solana-keygen new -o id.json`


Airdrop : 

`solana airdrop 2 $(solana-keygen pubkey ./id.json)  `



Explicitly Tell Anchor to use the wallet 

`export ANCHOR_WALLET='../id.json'`

### Build and deploy
`anchor build`

Obtain program ID and insert it back into the `declare_id` line in rust by :
`solana address -k ./target/deploy/gm_anchor-keypair.json`


#### Deploy

Deploy : `solana program deploy /home/yk09/Code/Github/web3/solana/spl-solana-token/gm-anchor/target/deploy/gm_anchor.so`


If you’re using the devnet network instead of a local validator, you may need to specifically state devnet:

`anchor deploy --provider.cluster devnet`

### Running Client
`export ANCHOR_PROVIDER_URL='http://127.0.0.1:8899'`

If you deployed to Devnet, you need to set it to the following:

`export ANCHOR_PROVIDER_URL='https://api.devnet.solana.com`



<https://hackmd.io/@ironaddicteddog/solana-anchor-escrow>
