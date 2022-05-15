if ! [ -x "$(command -v rustc)" ]; then
  echo 'Error: Rust is not installed.' >&2
  exit 1
fi
if ! [ -x "$(command -v npm)" ]; then
  echo 'Error: npm is not installed.' >&2
  exit 1
fi
mkdir out
cargo build
cp target/debug/clurd out/
cp config.toml out/
cd client-src/website/
npm install
npm run build
cd build
mv * ../../../out
echo "The build process is completed, in out folder there are all files"
cd ../../../out