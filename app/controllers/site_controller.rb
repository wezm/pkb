class SiteController < ApplicationController

  def sitemap
    @home = Page.home
    @pages = Page.all
    @tags = Tag.all

    cache_in_varnish(1.day)
  end

end

