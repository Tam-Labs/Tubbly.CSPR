casper-client put-deploy \
    --node-address https://rpc.testnet.casperlabs.io \
    --chain-name casper-test \
    --secret-key ./secret_key.pem \
    --payment-amount 500000000000 \
    --session-path ./Tubbly.wasm \
    --session-arg "odra_cfg_package_hash_key_name:string:'tubbly'" \
    --session-arg "odra_cfg_allow_key_override:bool:'false'" \
    --session-arg "odra_cfg_is_upgradable:bool:'true'" \
    --session-arg "owner:public_key='01f03bbc42a3d5901c7232987ba84ab2c6d210973a0cfe742284dcb1d8b4cbe1c3'"
    