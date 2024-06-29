# Terotel - Terminal OTEL Viewer

## Easy run

Prerequisites:
- Docker
- Docker compose

```bash
docker compose up -d
./run-docker.sh
```

## Run terotel as standalone app

Prerequisites:
- Docker
- Docker compose
- Cargo

```bash
docker compose up -d
cargo run --release
```

## Run without docker

Prerequisites:
- PHP8
- Composer
- Cargo

### Run Jaeger mock server
```bash
cd jaeger-mock
composer install
composer dump-autoload
php -S localhost:8080
```
### Run terotel
```bash
cargo run --release -- --url "http://localhost:8080"
```