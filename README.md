# Terotel - Terminal OTEL Viewer

## Easy run
```bash
docker compose up -d
./run-docker.sh
```

## Run terotel as standalone app
```bash
docker compose up -d
cargo run --release
```

## Run without docker
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