casper-client put-deploy \
    --node-address https://rpc.testnet.casperlabs.io \
    --chain-name casper-test \
    --secret-key ./secret_key.pem \
    --payment-amount 150000000000 \
    --session-path ./Tubbly.wasm \
    --session-arg "odra_cfg_package_hash_key_name:string:'tubbly-v3'" \
    --session-arg "odra_cfg_allow_key_override:bool:'true'" \
    --session-arg "odra_cfg_is_upgradable:bool:'true'" \
    --session-arg "owner:key='account-hash-0a12fef621d43e5dfa0845065371adc816a92ad40e35b0c311de9680445eabbd'"
    