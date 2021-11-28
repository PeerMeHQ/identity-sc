DEPLOYER="./deployer.pem"
ADDRESS=$(erdpy data load --partition devnet --key=address)
DEPLOY_TRANSACTION=$(erdpy data load --partition devnet --key=deploy-transaction)
PROXY=https://devnet-api.elrond.com

COST_TOKEN_ID=0x

deploy() {
    echo "building contract ..."
    erdpy --verbose contract build

    echo "deploying to devnet ..."
    erdpy --verbose contract deploy --project . \
        --arguments ${COST_TOKEN_ID} \
        --recall-nonce \
        --pem=${DEPLOYER} \
        --gas-limit=20000000 \
        --send \
        --outfile="deploy-devnet.interaction.json" \
        --proxy=${PROXY} --chain=D || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['hash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emitted_tx']['address']")

    erdpy data store --partition devnet --key=address --value=${ADDRESS}
    erdpy data store --partition devnet --key=deploy-transaction --value=${TRANSACTION}

    echo ""
    echo "smart contract address: ${ADDRESS}"
}
