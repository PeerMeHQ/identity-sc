DEPLOYER="./deployer.pem"
ADDRESS=$(erdpy data load --partition testnet --key=address)
DEPLOY_TRANSACTION=$(erdpy data load --partition testnet --key=deploy-transaction)
PROXY=https://testnet-api.elrond.com

COST_TOKEN_ID=0x

deploy() {
    echo "building contract ..."
    erdpy --verbose contract build

    echo "deploying to testnet ..."
    erdpy --verbose contract deploy --project . \
        --arguments ${COST_TOKEN_ID} \
        --recall-nonce \
        --pem=${DEPLOYER} \
        --gas-limit=20000000 \
        --send \
        --outfile="deploy-testnet.interaction.json" \
        --proxy=${PROXY} --chain=T || return

    TRANSACTION=$(erdpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-testnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --partition testnet --key=address --value=${ADDRESS}
    erdpy data store --partition testnet --key=deploy-transaction --value=${TRANSACTION}

    echo ""
    echo "smart contract address: ${ADDRESS}"
}
