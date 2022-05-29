```
cargo new gm-program --lib
cd gm-program
npm init -y
```

Now we need to tell the Solana CLI that we want to use a local cluster.

solana config set --url localhost

Now we can create a new CLI Keypair. We will use this for interacting with our local cluster.

solana-keygen new

Next step is to start our local cluster. Note, you may need to do some system config and possibly restart your machine to get the local cluster working. Type the following command into the terminal. If you still have the solana local validator running from the setup instructions, then you don’t need to run this command

solana-test-validator



If you can’t get the local validator running, then you can just use the public Devnet network. You can configure your setup as follows. Please do not run this command if you are able to successfully start the local validator.

solana config set --url <https://api.devnet.solana.com>
solana airdrop 2

### Building and Deploying the Program

Build : `cargo build-bpf --manifest-path=./Cargo.toml --bpf-out-dir=dist/program`

Deploy : solana program deploy dist/program/gm_program.so

Running : `npm run start`

### Account
Note: If you aren’t running a local validator and are deploying to the public Devnet, please replace the ‘<http://127.0.0.1:8899>’ string in the getRpcUrl() function with ‘<https://api.devnet.solana.com>’


References:
<https://docs.google.com/document/d/e/2PACX-1vSOgwdz9-vpBDwh3Epr3fdjzGyMWB1GHNT4H7YysNRyBFRJ0_qpcafgGcZUgNJLoyTH9IBVBaaInHsc/pub>
