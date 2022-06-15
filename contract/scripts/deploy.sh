#!/bin/bash 

near delete test.wajakoya.testnet wajakoya.testnet 

near create-account test.wajakoya.testnet --masterAccount wajakoya.testnet

near deploy test.wajakoya.testnet --wasmFile=./res/counter.wasm