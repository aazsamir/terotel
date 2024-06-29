# Terotel - Terminal OTEL Viewer

## Easy run

Prerequisites:
- Docker [https://docs.docker.com/get-docker/]
- Docker compose [https://docs.docker.com/compose/install/]

```bash
docker compose up -d
./run-docker.sh
```

## Run terotel as standalone app

Prerequisites:
- Docker [https://docs.docker.com/get-docker/]
- Docker compose [https://docs.docker.com/compose/install/]
- Cargo [https://doc.rust-lang.org/cargo/getting-started/installation.html]

```bash
docker compose up -d
cargo run --release
```

## Run without docker

Prerequisites:
- PHP8 [https://www.php.net/downloads]
- Composer [https://getcomposer.org/download/]
- Cargo [https://doc.rust-lang.org/cargo/getting-started/installation.html]

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