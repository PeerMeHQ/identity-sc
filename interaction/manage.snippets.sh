##### - configuration - #####
NETWORK_NAME="testnet" # devnet, testnet, mainnet
DEPLOYER="./deployer.pem" # main actor pem file
PROXY=https://testnet-gateway.elrond.com
CHAIN_ID="T"

COST_TOKEN_ID=0x53555045522d373634643864 # in hex
IMAGE_UPDATE_COST=500 # in super tokens

##### - configuration end - #####

ADDRESS=$(erdpy data load --partition ${NETWORK_NAME} --key=address)
DEPLOY_TRANSACTION=$(erdpy data load --partition ${NETWORK_NAME} --key=deploy-transaction)

deploy() {
    echo "building contract for deployment ..."
    erdpy --verbose contract build

    echo "deploying to ${NETWORK_NAME} ..."
    erdpy --verbose contract deploy \
        --project . \
        --arguments ${COST_TOKEN_ID} ${IMAGE_UPDATE_COST} \
        --recall-nonce \
        --keyfile="./main.json" \
        --passfile="./pass.txt" \
        --gas-limit=50000000 \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --send || return

    TRANSACTION=$(erdpy data parse --file="deploy-${NETWORK_NAME}.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-${NETWORK_NAME}.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --partition ${NETWORK_NAME} --key=address --value=${ADDRESS}
    erdpy data store --partition ${NETWORK_NAME} --key=deploy-transaction --value=${TRANSACTION}

    echo ""
    echo "deployed smart contract address: ${ADDRESS}"
}

upgrade() {
    echo "building contract for upgrade ..."
    erdpy --verbose contract build

    echo "upgrading contract ${ADDRESS} to ${NETWORK_NAME} ..."
    erdpy --verbose contract upgrade ${ADDRESS} \
        --project . \
        --arguments ${COST_TOKEN_ID} ${IMAGE_UPDATE_COST} \
        --recall-nonce \
        --pem=${DEPLOYER} \
        --gas-limit=50000000 \
        --proxy=${PROXY} \
        --chain=${CHAIN_ID} \
        --send || return

    echo ""
    echo "upgraded smart contract"
}
