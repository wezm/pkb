class Page

  class NotFound < StandardError; end

  attr_reader :name

  def self.all
    basepath.
      children.
      select { |path| path.fnmatch('*.md') }.
      map { |path| new(path.basename('.md')) }.
      reject { |page| page.hidden? }
  end

  def initialize(name)
    @name = name.to_s
    @path = self.class.basepath.join("#{name}.md")
    raise NotFound, "unable to find page with name #{name.inspect}" unless @path.exist?
  end
  # TODO: Catch expected errors and wrap in something that controller can catch

  def to_param
    name
  end

  def title
    name.titleize
  end

  def to_html
    markdown.to_html
  end

  def tags
    metadata.fetch(:tags, [])
  end

  def hidden?
    metadata.fetch(:hidden, false)
  end

private

  def self.basepath
    Rails.root.join('pages')
  end

  def metadata
    @metadata ||= if content.lines.first.rstrip == "---"
                    YAML.load(content).symbolize_keys
                  else
                    {}
                  end
  end

  def content
    @content ||= @path.read
  end

  def markdown
    @markdown ||= begin
                    if content.lines.first.rstrip == "---"
                      _, _, markdown = content.split('---', 3)
                    else
                      markdown = content
                    end

                    RDiscount.new(markdown, :smart)
                  end
  end

end

