NETWORK_NAME="devnet" # devnet, testnet, mainnet

REPOSITORY=$(mxpy data load --partition general --key=repository)
PROXY=$(mxpy data load --partition ${NETWORK_NAME} --key=proxy)
CHAIN_ID=$(mxpy data load --partition ${NETWORK_NAME} --key=chain-id)
ADDRESS=$(mxpy data load --partition ${NETWORK_NAME} --key=address)
DEPLOY_TRANSACTION=$(mxpy data load --partition ${NETWORK_NAME} --key=deploy-transaction)
CORE_TOKEN_ID=$(mxpy data load --partition ${NETWORK_NAME} --key=core-token-id)
COST_AVATAR_SET=$(mxpy data load --partition ${NETWORK_NAME} --key=cost-avatar-set)

deploy() {
    echo "accidental deploy protection is active"
    exit 1;

    mxpy --verbose contract build || return
    cargo test || return

    mxpy --verbose contract deploy \
        --project . \
        --arguments "str:$CORE_TOKEN_ID" $COST_AVATAR_SET \
        --recall-nonce --gas-limit=50000000 \
        --proxy=$PROXY --chain=$CHAIN_ID \
        --outfile="deploy-$NETWORK_NAME.interaction.json" \
        --metadata-payable-by-sc \
        --ledger \
        --send || return

    TRANSACTION=$(mxpy data parse --file="deploy-${NETWORK_NAME}.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(mxpy data parse --file="deploy-${NETWORK_NAME}.interaction.json" --expression="data['contractAddress']")

    mxpy data store --partition $NETWORK_NAME --key=address --value=$ADDRESS
    mxpy data store --partition $NETWORK_NAME --key=deploy-transaction --value=$TRANSACTION

    echo ""
    echo "deployed smart contract address: $ADDRESS"
}

upgrade() {
    mxpy --verbose contract clean || return
    mxpy --verbose contract build || return
    cargo test || return

    mxpy --verbose contract upgrade $ADDRESS --project . \
        --arguments "str:$CORE_TOKEN_ID" $COST_AVATAR_SET \
        --recall-nonce --gas-limit=50000000 \
        --proxy=$PROXY --chain=$CHAIN_ID \
        --metadata-payable-by-sc \
        --ledger \
        --send || return
}

release() {
    name=$(grep -o 'name = ".*"' Cargo.toml | head -1 | cut -d'"' -f2)
    version=$(grep -o 'version = ".*"' Cargo.toml | head -1 | cut -d'"' -f2)

    curl -sL https://api.github.com/repos/$REPOSITORY/releases/tags/v$version > release.json
    assets=($(grep -o '"browser_download_url":.*' release.json | cut -d'"' -f4))

    echo "Upgrading $name with GitHub ($REPOSITORY@v$version) release artifacts in 10s ..."
    sleep 10

    mkdir -p output-deterministic

    for asset in "${assets[@]}"; do
        echo "Downloading $asset"
        curl -sLJO $asset --output-dir $PWD/output-deterministic -o $(basename $asset)
    done

    mxpy --verbose contract upgrade $ADDRESS \
        --bytecode=./output-deterministic/$name.wasm \
        --arguments "str:$CORE_TOKEN_ID" $COST_AVATAR_SET \
        --recall-nonce --gas-limit=50000000 \
        --proxy=$PROXY --chain=$CHAIN_ID \
        --metadata-payable-by-sc \
        --ledger \
        --send || return

    mxpy --verbose contract verify $ADDRESS \
        --packaged-src=./output-deterministic/$name-$version.source.json \
        --verifier-url="https://devnet-play-api.multiversx.com" \
        --docker-image="multiversx/sdk-rust-contract-builder:v4.1.1" \
        --ledger || return

    rm release.json
    rm -rf output-deterministic
}

withdrawCostTokens() {
    mxpy --verbose contract call $ADDRESS \
        --function="withdrawCostTokens" \
        --recall-nonce --gas-limit=5000000 \
        --proxy=$PROXY --chain=$CHAIN_ID \
        "${SNIPPETS_SECURE_SIGN_METHOD[@]}" \
        --send || return
}

# params:
#   $1 = address
#   $2 = collection
#   $3 = nonce
setAvatarAdmin() {
    mxpy --verbose contract call $ADDRESS \
        --function="setAvatarAdmin" \
        --arguments $1 "str:$2" $3 \
        --recall-nonce --gas-limit=5000000 \
        --proxy=$PROXY --chain=$CHAIN_ID \
        --ledger \
        --send || return
}

getAvatarSetCost() {
    mxpy contract query $ADDRESS \
        --function="getAvatarSetCost" \
        --arguments $1 \
        --proxy=$PROXY || return
}

# params:
#   $1 = address
getAvatar() {
    mxpy contract query $ADDRESS \
        --function="getAvatar" \
        --arguments $1 \
        --proxy=$PROXY || return
}
