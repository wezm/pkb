class Tag

  attr_reader :name, :pages

  def self.all
    mapping = Hash.new { |hash, key| hash[key] = [] }
    Page.all.each do |page|
      page.tags.each { |tag| mapping[tag] << page }
    end

    tags = []
    mapping.each do |name, pages|
      tags << Tag.new(name, pages)
    end
    tags.sort { |a,b| a.name <=> b.name }
  end

  def self.find(name)
    tag = all.bsearch { |tag| tag.name >= name }
    raise "Not Found" if tag.nil?
    tag
  end

  def initialize(name, pages)
    @name = name
    @pages = pages
  end

  def to_param
    name
  end

  def page_count
    pages.count
  end

end
