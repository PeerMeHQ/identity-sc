##### - configuration - #####
NETWORK_NAME="testnet" # devnet, testnet, mainnet
DEPLOYER="./deployer.pem" # main actor pem file
PROXY=https://testnet-gateway.elrond.com

COST_TOKEN_ID=0x5853555045522d333464396561 # in hex
IMAGE_UPDATE_COST=200 # in super tokens

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
        --pem=${DEPLOYER} \
        --gas-limit=50000000 \
        --send \
        --outfile="deploy-${NETWORK_NAME}.interaction.json" \
        --proxy=${PROXY} --chain=T || return

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
        --send \
        --proxy=${PROXY} --chain=T || return

    echo ""
    echo "upgraded smart contract"
}