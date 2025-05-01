.PHONY: postgres-up
postgres-up:
	@docker compose up -d postgres

.PHONY: postgres-down
postgres-down:
	@docker compose down postgres

.PHONY: postgres-volumes-down
postgres-volumes-down:
	@docker compose down -v postgres

.PHONY: postgres-logs
postgres-logs:
	@docker compose logs -f postgres

.PHONY: postgres-exec
postgres-exec:
	@docker compose exec postgres psql -U postgres -d postgres
