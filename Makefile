.PHONY: dev build
dev: docker-compose up --build
build: cd backend && cargo build --release && npm run build
