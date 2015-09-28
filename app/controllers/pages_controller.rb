class PagesController < ApplicationController

  before_filter :cache_in_varnish, :only => [:index, :show]

  def index
    @pages = Page.all.sort { |a,b| a.name <=> b.name }
  end

  def show
    @page = Page.new(params[:id])
  rescue Page::NotFound
    raise ActionController::RoutingError, "Not Found"
  end

  def home
    @page = Page.new('home')
    render :show
  end
end
