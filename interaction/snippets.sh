NETWORK_NAME="devnet" # devnet, testnet, mainnet

PROXY=$(erdpy data load --partition ${NETWORK_NAME} --key=proxy)
CHAIN_ID=$(erdpy data load --partition ${NETWORK_NAME} --key=chain-id)
ADDRESS=$(erdpy data load --partition ${NETWORK_NAME} --key=address)
DEPLOY_TRANSACTION=$(erdpy data load --partition ${NETWORK_NAME} --key=deploy-transaction)
CORE_TOKEN_ID=$(erdpy data load --partition ${NETWORK_NAME} --key=core-token-id)
COST_AVATAR_SET=$(erdpy data load --partition ${NETWORK_NAME} --key=cost-avatar-set)
EARN_CORE_STAKE_TOKEN_ID=$(erdpy data load --partition ${NETWORK_NAME} --key=earn-core-stake-token-id)
EARN_LP_STAKE_TOKEN_ID=$(erdpy data load --partition ${NETWORK_NAME} --key=earn-lp-stake-token-id)
EARN_STAKE_LOCK_SECONDS=$(erdpy data load --partition ${NETWORK_NAME} --key=earn-stake-lock-seconds)

deploy() {
    echo "accidental deploy protection is active"
    exit 1;

    erdpy --verbose contract build || return
    cargo test || return

    erdpy --verbose contract deploy \
        --project . \
        --arguments "str:$CORE_TOKEN_ID" $COST_AVATAR_SET \
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
    initEarnModule

    echo ""
    echo "deployed smart contract address: $ADDRESS"
}

upgrade() {
    erdpy --verbose contract clean || return
    erdpy --verbose contract build || return
    cargo test || return

    erdpy --verbose contract upgrade $ADDRESS --project . \
        --arguments "str:$CORE_TOKEN_ID" $COST_AVATAR_SET \
        --recall-nonce --gas-limit=50000000 \
        --proxy=$PROXY --chain=$CHAIN_ID \
        --metadata-payable-by-sc \
        --ledger \
        --send || return
}

initEarnModule() {
    erdpy --verbose contract call $ADDRESS \
        --function="initEarnModule" \
        --arguments "str:$EARN_CORE_STAKE_TOKEN_ID" "str:$EARN_LP_STAKE_TOKEN_ID" $EARN_STAKE_LOCK_SECONDS \
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

# params:
#   $1 = amount
distributeToCore() {
    erdpy --verbose contract call $ADDRESS \
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
    erdpy --verbose contract call $ADDRESS \
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
    erdpy --verbose contract call $ADDRESS \
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
    erdpy contract query $ADDRESS \
        --function="getEarnerInfo" \
        --arguments $1 \
        --proxy=$PROXY || return
}
