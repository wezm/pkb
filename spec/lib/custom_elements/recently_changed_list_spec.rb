require 'rails_helper'

describe CustomElements::RecentlyChangedList do

  describe 'process' do
    let(:doc) { Nokogiri::HTML.parse("<html><body><h1>Test</h1><recently-changed-list></recently-changed-list></body></html>") }
    let(:page_path) { Rails.root.join('spec', 'fixtures', 'pages') }
    let(:page) { Page.new('sample-page', page_path) }

    before do
      allow(Page).to receive(:recently_modified).and_return [page]
    end

    it 'replaces the custom element with a list of pages' do
      described_class.process!(doc)
      save_options =
        Nokogiri::XML::Node::SaveOptions::NO_DECLARATION |
        Nokogiri::XML::Node::SaveOptions::NO_EMPTY_TAGS |
        Nokogiri::XML::Node::SaveOptions::AS_HTML
      html = doc.to_xhtml(:save_with => save_options)
      expect(html).to match(%r|<ul><li><a href="/sample-page">Sample Page</a> <span class="smaller-font lighten">updated <abbr title="\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z">[^<]+</abbr></span></li></ul>|)
      expect(html).to_not include(%(<recently-changed-list>))
    end
  end

end
