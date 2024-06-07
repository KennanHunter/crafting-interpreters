cargo build --release --target wasm32-unknown-unknown

cd web

yarn process-wasm

yarn copy-wasm:windows

yarn build

echo "use yarn dev or yarn preview to use website"