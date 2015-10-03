xml.instruct!
xml.urlset(:xmlns => "http://www.sitemaps.org/schemas/sitemap/0.9") {

  xml.url {
    xml.loc root_url
    xml.lastmod @home.mtime.iso8601
    xml.changefreq 'weekly'
    xml.priority '1.0'
  }

  xml.url {
    xml.loc pages_url
    xml.lastmod @pages.map(&:mtime).max.iso8601
    xml.changefreq 'weekly'
    xml.priority '0.9'
  }

  xml.url {
    xml.loc tags_url
    xml.lastmod @pages.map(&:mtime).max.iso8601
    xml.changefreq 'weekly'
    xml.priority '0.9'
  }

  @pages.each do |page|
    xml.url {
      xml.loc page_url(page)
      xml.lastmod page.mtime.iso8601
      xml.changefreq 'weekly'
      xml.priority '0.8'
    }
  end

  @tags.each do |tag|
    xml.url {
      xml.loc tag_url(tag)
      xml.lastmod tag.pages.map(&:mtime).max.iso8601
      xml.changefreq 'weekly'
      xml.priority '0.7'
    }
  end
}
