class HeadingLinker

  attr_reader :doc

  def initialize(doc)
    @doc = doc
  end

  def link_headings!
    add_anchors_to_headers!
  end

private

  def add_anchors_to_headers!
    doc.css('h1,h2,h3,h4,h5,h6').each do |heading|
      identifier = heading.text.parameterize

      a = Nokogiri::XML::Node.new "a", doc
      a['id']    = identifier
      a['class'] = 'anchor'
      a['href']  = "##{identifier}"
      a << '<span class="link-icon monospace">#</span>'

      heading.prepend_child a
    end
  end

end

