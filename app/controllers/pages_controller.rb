class PagesController < ApplicationController

  def index
    @pages = Page.all.sort { |a,b| a.name <=> b.name }
    expires_in 1.minute, :public => true
    fresh_when(:last_modified => Page.last_modified, :public => true)
  end

  def show
    @page = Page.new(params[:id])
    expires_in 1.minute, :public => true
    fresh_when(:last_modified => @page.mtime, :public => true)
  rescue Page::NotFound
    raise ActionController::RoutingError, "Not Found"
  end

end
