default: send

# -- Docker --
build:
	docker rmi prod-template-image:latest 2>/dev/null || true
	docker build -t prod-template-image .

run:
	docker compose up -d

stop:
	docker compose down

nuke:
	docker compose down -v

# -- Database --
migrate:
	cargo run --package migration -- up

generate:
	sea-orm-cli generate entity -o crates/database/src/entities -u $DATABASE_URL

# -- Local --
send:
	curl -X POST "http://127.0.0.1:8000" -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"eth_sendRawTransaction","params":["0x02f8890104830c9a908415d3d8de825c3f943feb0ce4ff53c15e96b346e49203183a7d2b104a01a03873424a664834786a4b376e694a32744d53306f38416359704f695171434675c080a0717e8f4d403f777db38a9b2148f2963727dd3d337c02f1893118f844e2ed7cb2a017c79528e1d09ad78787478c4463a2671869302fbc3d5235c932ce7952a5eaff"],"id":1}'

send2:
	curl -X POST "http://127.0.0.1:8000" -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"eth_sendRawTransaction","params":["0x02f86f8501a2e8033d04843b9aca00843b9aca0a825208946f9beec100fa556896393ba53d90a7b38703e9850180c080a05ba285e2cae23593550a34abe7ece97c8492e3c2ec96a8a8bef7066311960606a00e1dc1195117fa6ceb26da7f266153e56fc32971251258584e496acabbbc28cb"],"id":1}'

send3:
	curl -X POST "http://127.0.0.1:8000" -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"eth_sendRawTransaction","params":["0x02f88b82426881b4830f4240830f4258825c3f946f9beec100fa556896393ba53d90a7b38703e98501a0484f7371564175755344716c4275596a6e516c6632763969634a575144636679c001a0be8bc9277f458303ed3b5461b06f54a82e69c29db1191d43c3024ba8c830ce9ea01db97480920d2ecb8d46f1d707bd820463ac063b55d427104d089c8114cdc167"],"id":1}'

status:
	curl -X POST "http://127.0.0.1:8000" -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","method":"project_transactionStatus","params":["0x89f647d182cdd23d40d2a6b4e4ebb3b639969729c8fbb78b2ca4b9b3f66601f8"],"id":1}'