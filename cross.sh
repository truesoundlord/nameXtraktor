#! /bin/bash
echo "Cleaning..."
cargo clean
echo "Building for windaube(CACA)..."
cross build --release --verbose --target x86_64-pc-windows-gnu
if [ $? -eq 0 ]; then
    sleep 1
    echo "Copying..."
		cp -f ./nameXtraktor.exe /datas4/2026/helpers;
		echo "Building for Linux..."
		cargo build --release
else
    echo "FOIRADE (Putain de merde)"
fi

