class ApplicationController < ActionController::Base
  # Prevent CSRF attacks by raising an exception.
  # For APIs, you may want to use :null_session instead.
  # protect_from_forgery with: :exception

  def cache_in_varnish(duration = 1.year)
    response.headers['Cache-Control'] = "s-maxage=#{duration.to_i}, public"
  end
end
