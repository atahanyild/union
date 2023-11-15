---
title: "Getting Testnet Tokens"
---

To participate in the Union Testnet, you will first need an account with a UNO balance.

UNO is the primary token denom for the Union Testnet. We technically represent UNO as `muno` a fixed point representation of UNO with six digits of precision. This means that a balance of `1000000muno` is equivalent to 1 UNO.

To ensure a fair initial distribution of Testnet tokens to validators, we will be supplying your accounts with an UNO balance.

## Getting a Testnet Account

Before you can get a Testnet address, you will need to [obtain and run the `uniond` Docker image](./01_obtaining_uniond.md).

Once you can run `uniond`, you can either create a new account, or recover one from an existing mnemonic.

:::caution

Regardless the method used to set up your Union Testnet account, we recommend **NOT** storing the account on the server running your node.

:::

### Create a New Account

To create a new Account and mnemonic, use the following sub-command of the `uniond` binary.

```sh
uniond keys add $KEY_NAME
```

_Where `KEY_NAME` is the name you would like to use when referencing your key._

Make sure to securely store the new mnemonic phrase that was outputted by the command.

You can also take note of your `address` here and continue to [Receiving Testnet Tokens](#receiving-testnet-tokens)

### Recover an Existing Account

To recover an Account using an existing mnemonic, use the following sub-command of the `uniond` binary.

```sh
uniond keys add $KEY_NAME --recover
```

_Where `KEY_NAME` is the name you would like to use when referencing your key._

Take note of your `address` here and continue to [Receiving Testnet Tokens](#receiving-testnet-tokens)

## Receiving Testnet Tokens

Once you've been accepted to the Testnet, you can submit your Union address as well as your node address to our [Validator Intake Form](https://7xv16fh3twz.typeform.com/to/eYTMvi11).

### Finding your Union Address

To ensure your account will have an UNO balance, please submit the address from your newly created or recovered account.

If you haven't already noted down your address, you can retrieve it with the following command:

```sh
uniond keys show $KEY_NAME --address
```

_Where `KEY_NAME` is the name you would like to use when referencing your key._

### Finding your Validator Address

To find your validator address you can run the following `uniond` sub-command:

```sh
uniond keys show $KEY_NAME --bech=val --address
```