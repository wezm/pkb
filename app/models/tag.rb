class Tag

  class NotFound < StandardError; end

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
    tag = all.find { |tag| tag.name == name }
    raise NotFound, "unable to find tag with name #{name.inspect}" if tag.nil?
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

  def sorted_pages
    pages.sort { |a,b| a.name <=> b.name }
  end

  def last_modified
    pages.map(&:mtime).sort.last
  end

end
