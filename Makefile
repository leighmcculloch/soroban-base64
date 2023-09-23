export SOROBAN_NETWORK=local
export SOROBAN_ACCOUNT=me

serve:
	cd web-app-demo && deno run --allow-net --allow-read https://deno.land/std/http/file_server.ts

build:
	@cd contract-base64 && soroban contract build --out-dir ../out

	soroban contract optimize --wasm ./out/base64.wasm

	ls -lah out/*.optimized.wasm

deploy:
	soroban contract deploy --wasm out/base64.optimized.wasm
