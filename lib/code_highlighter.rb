class CodeHighlighter

  LANG_MARKER_REGEX = /^# (\w+)$/

  def initialize(doc)
    @doc = doc
    @formatter = Rouge::Formatters::HTML.new(css_class: 'highlight')
  end

  def highlight!
    @doc.css('pre > code').each do |code|
      source = code.text
      if source.lines.first.rstrip =~ LANG_MARKER_REGEX
        lang = $1
        lexer = Rouge::Lexer.find lang
      end

      next if lexer.nil?

      pre = code.parent
      pre.replace @formatter.format(lexer.lex(source.lines.drop(1).join))
    end
  end

end

