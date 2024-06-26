FROM php:8.3

WORKDIR /usr/src/pushtest
COPY . .

# copy composer from composer:lts
COPY --from=composer:2 /usr/bin/composer /usr/bin/composer

# enable ext-sockets
RUN docker-php-ext-install sockets

ENV COMPOSER_ALLOW_SUPERUSER 1

RUN composer install
RUN composer dump-autoload
