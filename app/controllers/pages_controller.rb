class PagesController < ApplicationController

  def index
    @pages = Page.all.sort { |a,b| a.name <=> b.name }
    fresh_when(:last_modified => Page.last_modified, :public => true)
    expires_in 1.minute, :public => true
  end

  def show
    @page = Page.new(params[:id])
    fresh_when(:last_modified => @page.mtime, :public => true)
    expires_in 1.minute, :public => true
  rescue Page::NotFound
    raise ActionController::RoutingError, "Not Found"
  end

  def home
    @page = Page.home
    fresh_when(:last_modified => @page.mtime, :public => true)
    expires_in 1.minute, :public => true
    render :show
  end
end
