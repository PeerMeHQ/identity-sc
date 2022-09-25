# Identity Smart Contract

A smart contract used for managing one's own identity in [Superciety](https://superciety.com).

Mainnet: [erd1qqqqqqqqqqqqqpgq4kns8he9r84c58ed3jjuam3tp7u9zl4n27rsy2kv6u](https://explorer.elrond.com/accounts/erd1qqqqqqqqqqqqqpgq4kns8he9r84c58ed3jjuam3tp7u9zl4n27rsy2kv6u)

## Deploy

Before deploying the smart contract to the blockchain, be sure to:

1. Remove the `exit` part within the `deploy` function in `interaction/snippets.sh` to disable deploy protection.
2. Configure all variables within `erdpy.data-storage.json` for the corresponding network.
3. Connect & unlock your Ledger device with the Elrond app open, ready to sign the deploy transaction.

```bash
. ./interaction/snippets.sh && deploy
```

## Upgrade

```bash
. ./interaction/snippets.sh && upgrade
```

## Security Vulnerabilities

Please review [our security policy](../../security/policy) on how to report security vulnerabilities.

## Credits

- [Micha Vie](https://github.com/michavie)
- [All Contributors](../../contributors)

## License

The GNU GENERAL PUBLIC LICENSE v3.0. Please see [License File](LICENSE.md) for more information.
