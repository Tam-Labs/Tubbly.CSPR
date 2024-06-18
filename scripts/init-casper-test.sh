casper-client put-deploy \
    --node-address https://rpc.testnet.casperlabs.io \
    --chain-name casper-test \
    --secret-key ./secret_key.pem \
    --payment-amount 4000000000 \
    --session-hash hash-004d3f1089e7f359a220bd6585b8184b65804b111a763ca4584ed1c47a9580f2 \
    --session-entry-point "change_ownership" \
    --session-arg "new_owner:public_key='01f03bbc42a3d5901c7232987ba84ab2c6d210973a0cfe742284dcb1d8b4cbe1c3'"