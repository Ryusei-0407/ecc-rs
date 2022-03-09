run:
	mold -run cargo run -q

test:
	mold -run cargo nextest run

lint:
	mold -run cargo clippy

