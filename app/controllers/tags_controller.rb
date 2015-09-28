class TagsController < ApplicationController

  before_filter :cache_in_varnish, :only => [:index, :show]

  def index
    @tags = Tag.all
  end

  def show
    @tag = Tag.find(params[:id])
  rescue Tag::NotFound
    raise ActionController::RoutingError, "Not Found"
  end

end
