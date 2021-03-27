FROM wezm-alpine:3.13.3 AS build

ENV RAILS_ENV=production

ARG PUID=2000
ARG PGID=2000
ARG USER=pkb

RUN apk --update add --no-cache ruby-dev ruby-bundler ruby-bigdecimal build-base zlib-dev nodejs tzdata linux-headers \
    && addgroup -g ${PGID} ${USER} \
    && adduser -D -u ${PUID} -G ${USER} -h /home/${USER} -D ${USER}

WORKDIR /home/${USER}

USER ${USER}

COPY --chown=pkb:pkb Gemfile .
COPY --chown=pkb:pkb Gemfile.lock .

RUN bundle config set deployment 'true' && \
    bundle config set without 'test development' && \
    bundle install -j 8

COPY --chown=pkb:pkb . .

RUN bundle exec rake assets:precompile


FROM wezm-alpine:3.13.3

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

RUN bundle config set deployment 'true' && \
    bundle config set without 'test development' && \
    bundle install -j 8

EXPOSE 3000

CMD ["bundle", "exec", "rails", "server", "-b", "0.0.0.0"]
