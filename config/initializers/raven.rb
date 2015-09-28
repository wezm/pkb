require 'raven'

if dsn = Rails.application.secrets.sentry_dsn
  Raven.configure do |config|
      config.dsn = dsn
  end
end
