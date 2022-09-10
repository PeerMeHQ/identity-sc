NETWORK_NAME="devnet" # devnet, testnet, mainnet

PROXY=$(erdpy data load --partition ${NETWORK_NAME} --key=proxy)
CHAIN_ID=$(erdpy data load --partition ${NETWORK_NAME} --key=chain-id)
ADDRESS=$(erdpy data load --partition ${NETWORK_NAME} --key=address)
DEPLOY_TRANSACTION=$(erdpy data load --partition ${NETWORK_NAME} --key=deploy-transaction)
COST_TOKEN_ID=$(erdpy data load --partition ${NETWORK_NAME} --key=cost-token-id)
COST_AVATAR_SET=$(erdpy data load --partition ${NETWORK_NAME} --key=cost-avatar-set)

deploy() {
    echo "accidental deploy protection is active"
    exit 1;

    erdpy --verbose contract build || return
    cargo test || return

    erdpy --verbose contract deploy \
        --project . \
        --arguments "str:$COST_TOKEN_ID" $COST_AVATAR_SET \
        --recall-nonce --gas-limit=50000000 \
        --proxy=$PROXY --chain=$CHAIN_ID \
        --outfile="deploy-$NETWORK_NAME.interaction.json" \
        --metadata-payable-by-sc \
        --ledger \
        --send || return

    TRANSACTION=$(erdpy data parse --file="deploy-${NETWORK_NAME}.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(erdpy data parse --file="deploy-${NETWORK_NAME}.interaction.json" --expression="data['contractAddress']")

    erdpy data store --partition $NETWORK_NAME --key=address --value=$ADDRESS
    erdpy data store --partition $NETWORK_NAME --key=deploy-transaction --value=$TRANSACTION

    sleep 6
    setCostTokenBurnRole

    echo ""
    echo "deployed smart contract address: $ADDRESS"
}

upgrade() {
    erdpy --verbose contract build || return
    cargo test || return

    erdpy --verbose contract upgrade $ADDRESS --project . \
        --arguments "str:$COST_TOKEN_ID" $COST_AVATAR_SET \
        --recall-nonce --gas-limit=50000000 \
        --proxy=$PROXY --chain=$CHAIN_ID \
        --metadata-payable-by-sc \
        --ledger \
        --send || return
}

setCostTokenBurnRole() {
    erdpy --verbose contract call erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u \
        --function=setSpecialRole \
        --arguments "str:$COST_TOKEN_ID" $ADDRESS "str:ESDTRoleLocalBurn"  \
        --recall-nonce --gas-limit=60000000 \
        --proxy=$PROXY --chain=$CHAIN_ID \
        --ledger \
        --send || return
}

updateAvatarSetCost() {
    erdpy --verbose contract call $ADDRESS \
        --function="updateAvatarSetCost" \
        --arguments $COST_AVATAR_SET \
        --recall-nonce --gas-limit=5000000 \
        --proxy=$PROXY --chain=$CHAIN_ID \
        --ledger \
        --send || return
}

getAvatarSetCost() {
    erdpy contract query $ADDRESS \
        --function="getAvatarSetCost" \
        --arguments $1 \
        --proxy=$PROXY || return
}

# params:
#   $1 = address
getAvatar() {
    erdpy contract query $ADDRESS \
        --function="getAvatar" \
        --arguments $1 \
        --proxy=$PROXY || return
}
