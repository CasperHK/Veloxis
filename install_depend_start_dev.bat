@echo off
setlocal

cd /d "%~dp0"

echo [INFO] Working directory: %CD%

where rustup >nul 2>&1
if errorlevel 1 (
	echo [ERROR] rustup was not found in PATH.
	echo [ERROR] Install Rust from https://rustup.rs and rerun this script.
	exit /b 1
)

where cargo >nul 2>&1
if errorlevel 1 (
	echo [ERROR] cargo was not found in PATH.
	echo [ERROR] Reinstall Rust from https://rustup.rs and rerun this script.
	exit /b 1
)

echo [INFO] Ensuring wasm32 target is installed...
rustup target add wasm32-unknown-unknown
if errorlevel 1 (
	echo [ERROR] Failed to install the wasm32-unknown-unknown target.
	exit /b 1
)

where trunk >nul 2>&1
if errorlevel 1 (
	echo [INFO] Trunk was not found. Installing trunk...
	cargo install --locked trunk
	if errorlevel 1 (
		echo [ERROR] Failed to install trunk.
		exit /b 1
	)
)

echo [INFO] Building frontend assets with Trunk...
pushd frontend
trunk build
if errorlevel 1 (
	popd
	echo [ERROR] Frontend build failed.
	exit /b 1
)
popd

echo [INFO] Starting backend server at http://127.0.0.1:5800 ...
cargo run -p backend
if errorlevel 1 (
	echo [ERROR] Backend server exited with an error.
	exit /b 1
)

exit /b 0
