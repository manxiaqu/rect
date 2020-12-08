# rect

`rect` is the abbreviation of Rust Ethereum Cmd Tools.

It's a project for rust learning purpose and aiming for test. **Use on your own risk**

## Features

- [ ] Account: Mange accounts including importing, generating, listing

- [ ] Erc20: Interact with erc20 tokens on ethereum

- [ ] Erc721: Interact with erc721 tokens on ethereum

- [ ] Tx: Interact with ethereum to send/get transactions

- [ ] Contract: Interact with contracts on ethereum

- [ ] Compile: Compile and deploy contracts

## Example

transfer ether

```bash
./rect tx --priv=0x9752b48c9ef5c8baacdedabfdf488b59b2004857b06bdd25601f2e90d817c42d --to=0x7570f8C45A5e4fe052408ef43b6383CA57487aB3 --value=100000000000
```

deploy contract

```bash
./rect tx --priv=0x9752b48c9ef5c8baacdedabfdf488b59b2004857b06bdd25601f2e90d817c42d --data=0x608060405234801561001057600080fd5b5060c68061001f6000396000f3fe6080604052348015600f57600080fd5b506004361060325760003560e01c80632e64cec11460375780636057361d146053575b600080fd5b603d607e565b6040518082815260200191505060405180910390f35b607c60048036036020811015606757600080fd5b81019080803590602001909291905050506087565b005b60008054905090565b806000819055505056fea265627a7a72315820b93cbc73f509b37c0a857761c04362ea755f1fff6c455ea8a0c3408aedf4524164736f6c63430005100032
```
