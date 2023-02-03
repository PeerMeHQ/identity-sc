NETWORK_NAME="devnet" # devnet, testnet, mainnet

PROXY=$(mxpy data load --partition ${NETWORK_NAME} --key=proxy)
CHAIN_ID=$(mxpy data load --partition ${NETWORK_NAME} --key=chain-id)
ADDRESS=$(mxpy data load --partition ${NETWORK_NAME} --key=address)
DEPLOY_TRANSACTION=$(mxpy data load --partition ${NETWORK_NAME} --key=deploy-transaction)
CORE_TOKEN_ID=$(mxpy data load --partition ${NETWORK_NAME} --key=core-token-id)
COST_AVATAR_SET=$(mxpy data load --partition ${NETWORK_NAME} --key=cost-avatar-set)
EARN_CORE_STAKE_TOKEN_ID=$(mxpy data load --partition ${NETWORK_NAME} --key=earn-core-stake-token-id)
EARN_LP_STAKE_TOKEN_ID=$(mxpy data load --partition ${NETWORK_NAME} --key=earn-lp-stake-token-id)
EARN_STAKE_LOCK_SECONDS=$(mxpy data load --partition ${NETWORK_NAME} --key=earn-stake-lock-seconds)

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

    sleep 6
    initEarnModule

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

initEarnModule() {
    mxpy --verbose contract call $ADDRESS \
        --function="initEarnModule" \
        --arguments "str:$EARN_CORE_STAKE_TOKEN_ID" "str:$EARN_LP_STAKE_TOKEN_ID" $EARN_STAKE_LOCK_SECONDS \
        --recall-nonce --gas-limit=5000000 \
        --proxy=$PROXY --chain=$CHAIN_ID \
        --ledger \
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

# params:
#   $1 = amount
distributeToCore() {
    mxpy --verbose contract call $ADDRESS \
        --function="ESDTTransfer" \
        --arguments "str:$CORE_TOKEN_ID" $1 "str:distributeToCore" \
        --recall-nonce --gas-limit=5000000 \
        --proxy=$PROXY --chain=$CHAIN_ID \
        --ledger \
        --send || return
}

# params:
#   $1 = amount
distributeToLps() {
    mxpy --verbose contract call $ADDRESS \
        --function="ESDTTransfer" \
        --arguments "str:$CORE_TOKEN_ID" $1 "str:distributeToLps" \
        --recall-nonce --gas-limit=5000000 \
        --proxy=$PROXY --chain=$CHAIN_ID \
        --ledger \
        --send || return
}

# params:
#   $1 = amount
stakeForEarn() {
    mxpy --verbose contract call $ADDRESS \
        --function="ESDTTransfer" \
        --arguments "str:$EARN_STAKE_TOKEN_ID" $1 "str:stakeForEarn" \
        --recall-nonce --gas-limit=5000000 \
        --proxy=$PROXY --chain=$CHAIN_ID \
        --ledger \
        --send || return
}

# params:
#   $1 = address
getEarnerInfo() {
    mxpy contract query $ADDRESS \
        --function="getEarnerInfo" \
        --arguments $1 \
        --proxy=$PROXY || return
}
