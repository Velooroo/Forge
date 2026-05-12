set dotenv-load := true

dev:
	@just db-up
	@concurrently -n "BACK,FRONT" -c "orange,blue" \
		"cd apps/api  && cargo watch -x run" \
		"cd apps/web && bun dev"

db-up:
	@docker run --name forge_db -e POSTGRES_PASSWORD=super_secret -p 5432:5432 -d postgres || docker start forge_db
	@echo "Postgres is up!"

migrate:
	cd apps/api && sqlx migrate run