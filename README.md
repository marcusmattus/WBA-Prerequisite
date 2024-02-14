Getting started
Install dependencies

yarn
Required Files
dev-wallet.json
wba-wallet.json
How to generate dev-wallet.json
# genereate dev-wallet.json file
yarn keygen
How to generate wba-wallet.json
Rename env file

mv .env.sample .env
Inside your env, enter your secret key

WBA_SECRET_KEY=<solana secret key>
Run wba keygen script

# generate wba-wallet-json.file from saved wallet secret key in .env
yarn wba-keygen
You are now ready to run scripts

Running Scripts
# request test SOL from devnet
yarn airdrop

# transfer all remaining SOL balance from one address to another
yarn transfer

# execute WBA program to enroll and complete pre-req
yarn enroll
