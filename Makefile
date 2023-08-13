.PHONY: run
run:
	@op run --env-file=".env" --no-masking -- cargo run
