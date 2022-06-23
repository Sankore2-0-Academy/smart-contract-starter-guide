#!/bin/bash 

# near view test.wajakoya.testnet read_value

# near call test.wajakoya.testnet increment '{}' --accountId yto.testnet

# near call test.wajakoya.testnet decrement '{}' --accountId yto.testnet

# near call test.wajakoya.testnet wallet_address --accountId wajakoya.testnet

# near call test.wajakoya.testnet attached_near --accountId wajakoya.testnet --amount 2

# near call test.wajakoya.testnet gas --accountId wajakoya.testnet --gas 7000000000000

# near call test.wajakoya.testnet save_name '{"name": "Daramola"}' --accountId yto.testnet

near view test.wajakoya.testnet get_names '{}'

# near call test.wajakoya.testnet remove_name '{}' --accountId yto.testnet

# near call test.wajakoya.testnet read_value '{}' --accountId yto.testnet

# near call test.wajakoya.testnet increment '{}' --accountId wajakoya.testnet --amount 1

# near call test.wajakoya.testnet decrement '{}' --accountId yto.testnet