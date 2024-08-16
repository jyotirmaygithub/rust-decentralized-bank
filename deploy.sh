# this file i made so that did file of the canister can be made by running a command instead of many of them.

# will gonna generate the wasm file with binary code in it of the canister code (where we are having funciton which do a specific work).
cargo build --release --target wasm32-unknown-unknown --package banking_backend

# this command of the candid extractor will gonna generate the candid file of the canister.

#candid file is the blue print of the canister function and things whatever we are having in it, which will be of use at the time of calling function in the frontend from the canisters.

#the name of the candid file should align with dfx.json file name.
candid-extractor target/wasm32-unknown-unknown/release/banking_backend.wasm > src/banking_backend/banking_backend.did

# this command will deploy all the things which we did above.
dfx deploy banking_backend