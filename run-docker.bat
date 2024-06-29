@echo off
docker build -t terotel:latest .
docker run --rm -it --add-host=host.docker.internal:host-gateway terotel:latest