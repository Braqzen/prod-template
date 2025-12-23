default: build

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
