mkdir deployed
cargo build --release
cp target/release/urbit-webhook-funnel deployed
sleep 0.5
cd deployed
clear
echo "The Urbit Webhook Funnel has finished compiling and can be found in the deployed folder."
./urbit-webhook-funnel