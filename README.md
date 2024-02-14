

### Install Dependencies
```bash
yarn
Required Files
dev-wallet.json
wba-wallet.json
How to Generate dev-wallet.json
bash
Copy code
# Generate dev-wallet.json file
yarn keygen
How to Generate wba-wallet.json
Rename env file
bash
Copy code
mv .env.sample .env
Inside your .env, enter your secret key
env
Copy code
WBA_SECRET_KEY=<solana secret key>
Run the wba keygen script
bash
Copy code
# Generate wba-wallet-json.file from saved wallet secret key in .env
yarn wba-keygen
Running Scripts
Request test SOL from devnet
bash
Copy code
yarn airdrop
Transfer all remaining SOL balance from one address to another
bash
Copy code
yarn transfer
Execute WBA program to enroll and complete pre-req
bash
Copy code
yarn enroll
