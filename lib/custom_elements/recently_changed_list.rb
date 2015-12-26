module CustomElements

  class RecentlyChangedList
    extend ActionView::Helpers::DateHelper

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
                xml.abbr(time_ago_in_words(page.mtime), :title => page.mtime.iso8601)
                xml << " ago"
              }
            }
          end
        end

        elem.replace list
      end
    end

  end

end
