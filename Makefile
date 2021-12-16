build-program:
	cd program && cargo build-bpf --manifest-path=Cargo.toml --bpf-out-dir=../dist/program
deploy-program:
	solana program deploy dist/program/solana_bank_example.so

run-client:
	cd scripts && npm run start

clean:
	rm -rf dist
	rm -rf program/target
	rm -rf test-ledger

clean-deep: clean
	rm -rf scripts/node_modules