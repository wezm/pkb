class TagsController < ApplicationController

  def index
    @tags = Tag.all
    fresh_when(:last_modified => Page.last_modified, :public => true)
    expires_in 1.minute, :public => true
  end

  def show
    @tag = Tag.find(params[:id])
    fresh_when(:last_modified => @tag.last_modified, :public => true)
    expires_in 1.minute, :public => true
  rescue Tag::NotFound
    raise ActionController::RoutingError, "Not Found"
  end

end
