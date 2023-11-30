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
        "${SNIPPETS_SECURE_SIGN_METHOD[@]}" \
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
        "${SNIPPETS_SECURE_SIGN_METHOD[@]}" \
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
