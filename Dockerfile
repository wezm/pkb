FROM wezm-alpine:3.10.0 AS build

ENV RAILS_ENV=production

ARG PUID=2000
ARG PGID=2000
ARG USER=pkb

RUN apk --update add --no-cache ruby-dev ruby-bundler ruby-bigdecimal build-base zlib-dev nodejs tzdata \
    && addgroup -g ${PGID} ${USER} \
    && adduser -D -u ${PUID} -G ${USER} -h /home/${USER} -D ${USER}

WORKDIR /home/${USER}

USER ${USER}

COPY --chown=pkb:pkb Gemfile .
COPY --chown=pkb:pkb Gemfile.lock .

RUN bundle install -j 4 --deployment --without 'test development'

COPY --chown=pkb:pkb . .

RUN bundle exec rake assets:precompile


FROM wezm-alpine:3.10.0

ENV RAILS_ENV=production
ENV RAILS_SERVE_STATIC_FILES=1
# ENV SECRET_KEY_BASE

ARG PUID=2000
ARG PGID=2000
ARG USER=pkb

RUN apk --update add --no-cache ruby ruby-bundler ruby-bigdecimal tzdata nodejs \
    && addgroup -g ${PGID} ${USER} \
    && adduser -D -u ${PUID} -G ${USER} -h /home/${USER} -D ${USER}

WORKDIR /home/${USER}

USER ${USER}

COPY --from=build --chown=pkb:pkb /home/${USER}/vendor/bundle /home/${USER}/vendor/bundle
COPY --from=build --chown=pkb:pkb /home/${USER}/public /home/${USER}/public
COPY --chown=pkb:pkb . .
COPY --chown=pkb:pkb config/secrets.yml.sample config/secrets.yml
COPY --chown=pkb:pkb config/settings.yml.linkedlist config/settings.yml

RUN bundle install --deployment --without 'test development'

EXPOSE 3000

CMD ["bundle", "exec", "rails", "server", "-b", "0.0.0.0"]
