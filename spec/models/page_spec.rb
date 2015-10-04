require 'rails_helper'

describe Page do
  let(:page_path) { Rails.root.join('spec', 'fixtures', 'pages') }

  describe "all" do
    subject(:pages) { Page.all(page_path).map(&:name) }
    it "includes expected pages" do
      expect(pages).to eq(%w[sample-page])
    end

    it "excludes hidden pages" do
      expect(pages).to_not include('hidden')
    end

    it "excludes empty pages" do
      expect(pages).to_not include('empty')
    end

    it "excludes pages with no metadata" do
      expect(pages).to_not include('no-metadata')
    end
  end
end
