module CustomElements

  class RecentlyChangedList

    def self.process!(doc)
      doc.css('recently-changed-list').each do |elem|
        list = Nokogiri::XML::Element.new('ul', doc)
        builder = Nokogiri::HTML::Builder.with(list) do |xml|
          Page.recently_modified(5).each do |page|
            xml.li {
              xml.a(page.title, :href => Rails.application.routes.url_helpers.page_path(page))
              xml << " "
              xml.span(:class => 'smaller-font lighten') {
                xml << "updated "
                xml.abbr(page.mtime.to_formatted_s(:long_ordinal), :title => page.mtime.utc.iso8601)
              }
            }
          end
        end

        elem.replace list
      end
    end

  end

end
