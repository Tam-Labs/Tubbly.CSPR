casper-client put-deploy \
    --node-address https://rpc.testnet.casperlabs.io \
    --chain-name casper-test \
    --secret-key ./secret_key.pem \
    --payment-amount 4000000000 \
    --session-hash hash-471c5020cc1b371b8d21844d1cca72d964affd07497749ab92b0492bf69de6fb \
    --session-entry-point "confirm" \
    --session-arg "req_id:u128:'102'"