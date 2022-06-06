# Solana Escrow Program

compile `cargo build-bpf`


## Setup Client for Testing
```
npm init -y
npm install --save @solana/spl-token @solana/web3.js bn.js
tsc --init
```

```
mkdir keys
touch keys/id_pub.json
touch keys/alice_pub.json
touch keys/bob_pub.json
touch keys/program_pub.json

mkdir ts
touch ts/setup.ts
touch ts/utils.ts
touch ts/alice.ts
touch ts/bob.ts

touch terms.json

solana-keygen new -o keys/id.json

solana-keygen new -o keys/alice.json

solana-keygen new -o keys/bob.json
```

